use std::io;
use std::path::Path;

use io::BufWriter;
use io::Write;

use calamine::Data;
use calamine::Range;

use calamine::Reader;
use calamine::Xlsx;

pub fn range2rowcnt(rng: &Range<Data>) -> usize {
    let rows = rng.rows();
    rows.len()
}

pub fn range2cellcnt(rng: &Range<Data>) -> usize {
    rng.cells().len()
}

pub fn range2cellcnt_filtered<F>(rng: &Range<Data>, filter: &F) -> usize
where
    F: Fn(usize, usize, &Data) -> bool,
{
    rng.cells()
        .filter(|rcv| {
            let (row, col, val) = rcv;
            filter(*row, *col, val)
        })
        .count()
}

pub fn range2cellcnt_non_empty(rng: &Range<Data>) -> usize {
    range2cellcnt_filtered(rng, &|_, _, dat: &Data| dat != &Data::Empty)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Task {
    CountRows,
    CountBytes,
    CountCellsAll,
    CountCellsNonEmpty,
}

pub enum Stat {
    Rows(usize),
    Cells(usize),
    Bytes(u64),
}

impl Stat {
    pub fn to_writer<W>(&self, sheet_name: &str, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        match self {
            Self::Rows(r) => writeln!(wtr, "sheet:{sheet_name}\trows:{r}"),
            Self::Cells(c) => writeln!(wtr, "sheet:{sheet_name}\tcells:{c}"),
            Self::Bytes(b) => writeln!(wtr, "bytes:{b}"),
        }
    }
}

impl Stat {
    pub fn rng2rstat(rng: &Range<Data>) -> Self {
        let rowcnt: usize = range2rowcnt(rng);
        Self::Rows(rowcnt)
    }

    pub fn rng2cellcnt(rng: &Range<Data>) -> Self {
        let cellcnt: usize = range2cellcnt(rng);
        Self::Cells(cellcnt)
    }

    pub fn rng2cellcnt_filtered<F>(rng: &Range<Data>, filter: &F) -> Self
    where
        F: Fn(usize, usize, &Data) -> bool,
    {
        let cellcnt: usize = range2cellcnt_filtered(rng, filter);
        Self::Cells(cellcnt)
    }

    pub fn rng2cellcnt_non_empty(rng: &Range<Data>) -> Self {
        let cellcnt: usize = range2cellcnt_non_empty(rng);
        Self::Cells(cellcnt)
    }
}

impl Stat {
    pub fn path2fstat<P>(path2book: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let met: std::fs::Metadata = std::fs::metadata(path2book)?;
        let len: u64 = met.len();
        Ok(Self::Bytes(len))
    }
}

impl Task {
    pub fn rng2stat(&self, rng: &Range<Data>) -> Stat {
        match self {
            Self::CountRows => Stat::rng2rstat(rng),
            Self::CountCellsAll => Stat::rng2cellcnt(rng),
            Self::CountCellsNonEmpty => Stat::rng2cellcnt_non_empty(rng),
            _ => Stat::Bytes(0),
        }
    }
}

impl Task {
    pub fn book2stats2writer<P, W>(&self, path2book: P, mut swtr: W) -> Result<(), io::Error>
    where
        P: AsRef<Path>,
        W: FnMut(&str, &Stat) -> Result<(), io::Error>,
    {
        if Task::CountBytes.eq(self) {
            let stat: Stat = Stat::path2fstat(path2book)?;
            swtr("", &stat)?;
            return Ok(());
        }

        let p2b: &Path = path2book.as_ref();

        let mut xbk: Xlsx<_> = calamine::open_workbook(p2b)
            .map_err(|e| format!("unable to open the book {p2b:?}: {e}"))
            .map_err(io::Error::other)?;

        let sheet_names: Vec<String> = xbk.sheet_names();

        for sname in sheet_names {
            let rng: Range<Data> = xbk.worksheet_range(&sname).map_err(io::Error::other)?;
            let stat: Stat = self.rng2stat(&rng);
            swtr(&sname, &stat)?;
        }

        Ok(())
    }
}

impl Task {
    pub fn book2stats2iowriter<P, W>(&self, path2book: P, mut wtr: W) -> Result<(), io::Error>
    where
        P: AsRef<Path>,
        W: Write,
    {
        self.book2stats2writer(path2book, |sheet_name: &str, stat: &Stat| {
            stat.to_writer(sheet_name, &mut wtr)
        })?;
        wtr.flush()
    }
}

impl Task {
    pub fn book2stats2stdout<P>(&self, path2book: P) -> Result<(), io::Error>
    where
        P: AsRef<Path>,
    {
        let o = io::stdout();
        let mut ol = o.lock();
        self.book2stats2iowriter(path2book, BufWriter::new(&mut ol))?;
        ol.flush()
    }
}
