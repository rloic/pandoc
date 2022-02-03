use crate::definition::{Block, Caption, Cell, Citation, Inline, Meta, MetaValue, Pandoc, Row, TableBody, TableFoot, TableHead};
use crate::definition::Block::{BlockQuote, BulletList, DefinitionList, Div, Header, LineBlock, OrderedList, Para, Plain, Table};
use crate::definition::Inline::{Cite, Emph, Image, Link, Note, Quoted, SmallCaps, Span, Strikeout, Strong, Subscript, Superscript};
use crate::definition::MetaValue::{MetaBlocks, MetaInlines, MetaList, MetaMap};

pub type Inlines = Vec<Inline>;
pub type Blocks = Vec<Block>;

pub trait Walkable<T> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(T) -> T;
}



impl Walkable<Inline> for Pandoc {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        Pandoc {
            meta: self.meta.walk(f),
            blocks: self.blocks.walk(f),
            ..self
        }
    }
}

impl Walkable<Block> for Pandoc {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        Pandoc {
            meta: self.meta.walk(f),
            blocks: self.blocks.walk(f),
            ..self
        }
    }
}

impl Walkable<Pandoc> for Pandoc {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Pandoc) -> Pandoc {
        f(self)
    }
}

impl Walkable<Inlines> for Pandoc {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        Pandoc {
            meta: self.meta.walk(f),
            blocks: self.blocks.walk(f),
            ..self
        }
    }
}

impl Walkable<Blocks> for Pandoc {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        Pandoc {
            meta: self.meta.walk(f),
            blocks: self.blocks.walk(f),
            ..self
        }
    }
}

impl Walkable<Inline> for Meta {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|(k, v)| (k, v.walk(f))).collect()
    }
}

impl Walkable<Block> for Meta {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|(k, v)| (k, v.walk(f))).collect()
    }
}

impl Walkable<Meta> for Meta {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Meta) -> Meta {
        let tmp = self.into_iter().map(|(k, v)| (k, v.walk(f))).collect();
        f(tmp)
    }
}

impl Walkable<Inlines> for Meta {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|(k, v)| (k, v.walk(f))).collect()
    }
}

impl Walkable<Blocks> for Meta {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|(k, v)| (k, v.walk(f))).collect()
    }
}

impl Walkable<Inline> for MetaValue {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        match self {
            MetaMap(map) => MetaMap(map.walk(f)),
            MetaList(values) => MetaList(values.walk(f)),
            MetaInlines(values) => MetaInlines(values.walk(f)),
            MetaBlocks(values) => MetaBlocks(values.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Block> for MetaValue {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        match self {
            MetaMap(map) => MetaMap(map.walk(f)),
            MetaList(values) => MetaList(values.walk(f)),
            MetaInlines(values) => MetaInlines(values.walk(f)),
            MetaBlocks(values) => MetaBlocks(values.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Inlines> for MetaValue {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        match self {
            MetaMap(map) => MetaMap(map.walk(f)),
            MetaList(values) => MetaList(values.walk(f)),
            MetaInlines(values) => MetaInlines(values.walk(f)),
            MetaBlocks(values) => MetaBlocks(values.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Blocks> for MetaValue {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        match self {
            MetaMap(map) => MetaMap(map.walk(f)),
            MetaList(values) => MetaList(values.walk(f)),
            MetaInlines(values) => MetaInlines(values.walk(f)),
            MetaBlocks(values) => MetaBlocks(values.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Meta> for MetaValue {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Meta) -> Meta {
        match self {
            MetaMap(map) => MetaMap(map.walk(f)),
            MetaList(values) => MetaList(values.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Meta> for Vec<MetaValue> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Meta) -> Meta {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Vec<MetaValue> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Vec<MetaValue> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Vec<MetaValue> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Vec<MetaValue> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Block {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        match self {
            Plain(ils) => Plain(ils.walk(f)),
            Para(ils) => Para(ils.walk(f)),
            LineBlock(ilss) => LineBlock(ilss.into_iter().map(|it| it.walk(f)).collect()),
            BlockQuote(blks) => BlockQuote(blks.walk(f)),
            OrderedList(list_attributes, blkss) => OrderedList(list_attributes, blkss.into_iter().map(|it| it.walk(f)).collect()),
            BulletList(blkss) => BulletList(blkss.into_iter().map(|it| it.walk(f)).collect()),
            DefinitionList(definitions) => DefinitionList(definitions.into_iter().map(|(ils, blkss)| (ils.walk(f), blkss.into_iter().map(|it| it.walk(f)).collect())).collect()),
            Header(lvl, attr, ils) => Header(lvl, attr, ils.walk(f)),
            Table(attr, caption, spec, t_head, t_body, t_foot) => {
                Table(attr, caption.walk(f), spec, t_head.walk(f), t_body.walk(f), t_foot.walk(f))
            }
            Div(attr, blks) => Div(attr, blks.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Block> for Block {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        let tmp = match self {
            Plain(ils) => Plain(ils.walk(f)),
            Para(ils) => Para(ils.walk(f)),
            LineBlock(ilss) => LineBlock(ilss.into_iter().map(|it| it.walk(f)).collect()),
            BlockQuote(blks) => Block::BlockQuote(blks.walk(f)),
            OrderedList(list_attributes, blkss) => OrderedList(list_attributes, blkss.into_iter().map(|it| it.walk(f)).collect()),
            BulletList(blkss) => BulletList(blkss.into_iter().map(|it| it.walk(f)).collect()),
            DefinitionList(definitions) => DefinitionList(definitions.into_iter().map(|(ils, blkss)| (ils.walk(f), blkss.into_iter().map(|it| it.walk(f)).collect())).collect()),
            Header(lvl, attr, ils) => Header(lvl, attr, ils.walk(f)),
            Table(attr, caption, spec, t_head, t_body, t_foot) => {
                Table(attr, caption.walk(f), spec, t_head.walk(f), t_body.walk(f), t_foot.walk(f))
            }
            Div(attr, blks) => Div(attr, blks.walk(f)),
            _ => self
        };
        f(tmp)
    }
}

impl Walkable<Inlines> for Block {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        match self {
            Plain(ils) => Plain(ils.walk(f)),
            Para(ils) => Para(ils.walk(f)),
            LineBlock(ilss) => LineBlock(ilss.into_iter().map(|it| it.walk(f)).collect()),
            BlockQuote(blks) => Block::BlockQuote(blks.walk(f)),
            OrderedList(list_attributes, blkss) => OrderedList(list_attributes, blkss.into_iter().map(|it| it.walk(f)).collect()),
            BulletList(blkss) => BulletList(blkss.into_iter().map(|it| it.walk(f)).collect()),
            DefinitionList(definitions) => DefinitionList(definitions.into_iter().map(|(ils, blkss)| (ils.walk(f), blkss.into_iter().map(|it| it.walk(f)).collect())).collect()),
            Header(lvl, attr, ils) => Header(lvl, attr, ils.walk(f)),
            Table(attr, caption, spec, t_head, t_body, t_foot) => {
                Table(attr, caption.walk(f), spec, t_head.walk(f), t_body.walk(f), t_foot.walk(f))
            }
            Div(attr, blks) => Div(attr, blks.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Blocks> for Block {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        match self {
            Plain(ils) => Plain(ils.walk(f)),
            Para(ils) => Para(ils.walk(f)),
            LineBlock(ilss) => LineBlock(ilss.into_iter().map(|it| it.walk(f)).collect()),
            BlockQuote(blks) => Block::BlockQuote(blks.walk(f)),
            OrderedList(list_attributes, blkss) => OrderedList(list_attributes, blkss.into_iter().map(|it| it.walk(f)).collect()),
            BulletList(blkss) => BulletList(blkss.into_iter().map(|it| it.walk(f)).collect()),
            DefinitionList(definitions) => DefinitionList(definitions.into_iter().map(|(ils, blkss)| (ils.walk(f), blkss.into_iter().map(|it| it.walk(f)).collect())).collect()),
            Header(lvl, attr, ils) => Header(lvl, attr, ils.walk(f)),
            Table(attr, caption, spec, t_head, t_body, t_foot) => {
                Table(attr, caption.walk(f), spec, t_head.walk(f), t_body.walk(f), t_foot.walk(f))
            }
            Div(attr, blks) => Div(attr, blks.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Blocks> for Blocks {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        let tmp = self.into_iter().map(|it| it.walk(f)).collect();
        f(tmp)
    }
}

impl Walkable<Block> for Blocks {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Blocks {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Blocks {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Inline {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        let tmp = match self {
            Emph(ils) => Emph(ils.walk(f)),
            Strong(ils) => Strong(ils.walk(f)),
            Strikeout(ils) => Strikeout(ils.walk(f)),
            Superscript(ils) => Superscript(ils.walk(f)),
            Subscript(ils) => Subscript(ils.walk(f)),
            SmallCaps(ils) => SmallCaps(ils.walk(f)),
            Quoted(t, ils) => Quoted(t, ils.walk(f)),
            Cite(citations, ils) => Cite(citations.walk(f), ils.walk(f)),
            Link(attr, ils, target) => Link(attr, ils.walk(f), target),
            Image(attr, ils, target) => Image(attr, ils.walk(f), target),
            Note(blks) => Note(blks.walk(f)),
            Span(attr, ils) => Span(attr, ils.walk(f)),
            _ => self
        };
        f(tmp)
    }
}

impl Walkable<Block> for Inline {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        match self {
            Emph(ils) => Emph(ils.walk(f)),
            Strong(ils) => Strong(ils.walk(f)),
            Strikeout(ils) => Strikeout(ils.walk(f)),
            Superscript(ils) => Superscript(ils.walk(f)),
            Subscript(ils) => Subscript(ils.walk(f)),
            SmallCaps(ils) => SmallCaps(ils.walk(f)),
            Quoted(t, ils) => Quoted(t, ils.walk(f)),
            Cite(citations, ils) => Cite(citations.walk(f), ils.walk(f)),
            Link(attr, ils, target) => Link(attr, ils.walk(f), target),
            Image(attr, ils, target) => Image(attr, ils.walk(f), target),
            Note(blks) => Note(blks.walk(f)),
            Span(attr, ils) => Span(attr, ils.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Blocks> for Inline {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        match self {
            Emph(ils) => Emph(ils.walk(f)),
            Strong(ils) => Strong(ils.walk(f)),
            Strikeout(ils) => Strikeout(ils.walk(f)),
            Superscript(ils) => Superscript(ils.walk(f)),
            Subscript(ils) => Subscript(ils.walk(f)),
            SmallCaps(ils) => SmallCaps(ils.walk(f)),
            Quoted(t, ils) => Quoted(t, ils.walk(f)),
            Cite(citations, ils) => Cite(citations.walk(f), ils.walk(f)),
            Link(attr, ils, target) => Link(attr, ils.walk(f), target),
            Image(attr, ils, target) => Image(attr, ils.walk(f), target),
            Note(blks) => Note(blks.walk(f)),
            Span(attr, ils) => Span(attr, ils.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Inlines> for Inline {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        match self {
            Emph(ils) => Emph(ils.walk(f)),
            Strong(ils) => Strong(ils.walk(f)),
            Strikeout(ils) => Strikeout(ils.walk(f)),
            Superscript(ils) => Superscript(ils.walk(f)),
            Subscript(ils) => Subscript(ils.walk(f)),
            SmallCaps(ils) => SmallCaps(ils.walk(f)),
            Quoted(t, ils) => Quoted(t, ils.walk(f)),
            Cite(citations, ils) => Cite(citations.walk(f), ils.walk(f)),
            Link(attr, ils, target) => Link(attr, ils.walk(f), target),
            Image(attr, ils, target) => Image(attr, ils.walk(f), target),
            Note(blks) => Note(blks.walk(f)),
            Span(attr, ils) => Span(attr, ils.walk(f)),
            _ => self
        }
    }
}

impl Walkable<Inlines> for Inlines {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        let tmp = self.into_iter().map(|it| it.walk(f)).collect();
        f(tmp)
    }
}

impl Walkable<Inline> for Inlines {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Inlines {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Inlines {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}


impl Walkable<Inline> for Caption {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        Caption(self.0.map(|it| it.walk(f)), self.1.walk(f))
    }
}

impl Walkable<Block> for Caption {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        Caption(self.0.map(|it| it.walk(f)), self.1.walk(f))
    }
}

impl Walkable<Inlines> for Caption {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        Caption(self.0.map(|it| it.walk(f)), self.1.walk(f))
    }
}

impl Walkable<Blocks> for Caption {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        Caption(self.0.map(|it| it.walk(f)), self.1.walk(f))
    }
}

impl Walkable<Inline> for Row {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        Row(self.0, self.1.walk(f))
    }
}

impl Walkable<Block> for Row {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        Row(self.0, self.1.walk(f))
    }
}

impl Walkable<Inlines> for Row {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        Row(self.0, self.1.walk(f))
    }
}

impl Walkable<Blocks> for Row {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        Row(self.0, self.1.walk(f))
    }
}

impl Walkable<Inline> for Vec<Row> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Vec<Row> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Vec<Row> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Vec<Row> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for TableHead {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        TableHead(self.0, self.1.walk(f))
    }
}

impl Walkable<Block> for TableHead {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        TableHead(self.0, self.1.walk(f))
    }
}

impl Walkable<Inlines> for TableHead {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        TableHead(self.0, self.1.walk(f))
    }
}

impl Walkable<Blocks> for TableHead {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        TableHead(self.0, self.1.walk(f))
    }
}

impl Walkable<Inline> for TableBody {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        TableBody(self.0, self.1, self.2.walk(f), self.3.walk(f))
    }
}

impl Walkable<Block> for TableBody {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        TableBody(self.0, self.1, self.2.walk(f), self.3.walk(f))
    }
}

impl Walkable<Inlines> for TableBody {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        TableBody(self.0, self.1, self.2.walk(f), self.3.walk(f))
    }
}

impl Walkable<Blocks> for TableBody {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        TableBody(self.0, self.1, self.2.walk(f), self.3.walk(f))
    }
}

impl Walkable<Inline> for TableFoot {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        TableFoot(self.0, self.1.walk(f))
    }
}

impl Walkable<Block> for TableFoot {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        TableFoot(self.0, self.1.walk(f))
    }
}

impl Walkable<Inlines> for TableFoot {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        TableFoot(self.0, self.1.walk(f))
    }
}

impl Walkable<Blocks> for TableFoot {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        TableFoot(self.0, self.1.walk(f))
    }
}

impl Walkable<Inline> for Cell {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        Cell(self.0, self.1, self.2, self.3, self.4.walk(f))
    }
}

impl Walkable<Block> for Cell {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        Cell(self.0, self.1, self.2, self.3, self.4.walk(f))
    }
}

impl Walkable<Inlines> for Cell {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        Cell(self.0, self.1, self.2, self.3, self.4.walk(f))
    }
}

impl Walkable<Blocks> for Cell {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        Cell(self.0, self.1, self.2, self.3, self.4.walk(f))
    }
}

impl Walkable<Inline> for Vec<Cell> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Vec<Cell> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Vec<Cell> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Vec<Cell> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Citation {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        Citation {
            citation_prefix: self.citation_prefix.walk(f),
            citation_suffix: self.citation_suffix.walk(f),
            ..self
        }
    }
}

impl Walkable<Block> for Citation {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        Citation {
            citation_prefix: self.citation_prefix.walk(f),
            citation_suffix: self.citation_suffix.walk(f),
            ..self
        }
    }
}

impl Walkable<Inlines> for Citation {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        Citation {
            citation_prefix: self.citation_prefix.walk(f),
            citation_suffix: self.citation_suffix.walk(f),
            ..self
        }
    }
}

impl Walkable<Blocks> for Citation {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        Citation {
            citation_prefix: self.citation_prefix.walk(f),
            citation_suffix: self.citation_suffix.walk(f),
            ..self
        }
    }
}

impl Walkable<Inline> for Vec<Citation> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Vec<Citation> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Vec<Citation> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Vec<Citation> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inline> for Vec<TableBody> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inline) -> Inline {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Block> for Vec<TableBody> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Block) -> Block {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Inlines> for Vec<TableBody> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Inlines) -> Inlines {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}

impl Walkable<Blocks> for Vec<TableBody> {
    fn walk<F>(self, f: &mut F) -> Self where F: FnMut(Blocks) -> Blocks {
        self.into_iter().map(|it| it.walk(f)).collect()
    }
}