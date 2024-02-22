use crate::Result;

pub struct CrepeDB {}

impl CrepeDB {
    pub fn open(&self) -> Result<Self> {
        Ok(Self {})
    }

    pub fn open_readonly(&self) -> Result<Self> {
        Ok(Self {})
    }

    pub fn create_table(&self, name: &str) -> Result<()> {
        Ok(())
    }

    pub fn remove_table(&self, name: &str) -> Result<()> {
        Ok(())
    }
}
