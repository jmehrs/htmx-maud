use crate::Command;
use rusqlite::{Connection, Error};
use serde::Deserialize;

pub struct Device {
    pub id: u64,
    pub name: String,
    pub product: String,
    pub model: Option<String>,
    pub serial: Option<String>,
}

pub fn query_device(conn: &Connection, device_id: u64) -> Result<Device, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, product, model, serial
        FROM device
        WHERE id = ?1",
    )?;
    stmt.query_row((device_id,), |row| Ok(Device {
        id: row.get(0).expect("Couldn't cast id column to u64"),
        name: row.get(1).expect("Couldn't cast name column to String"),
        product: row.get(2).expect("Couldn't cast product column to String"),
        model: row.get(3).expect("Couldn't cast model column to String"),
        serial: row.get(4).expect("Couldn't cast serial column to String"),
    }))
}

impl Device {
    pub fn new(
        id: u64,
        name: String,
        product: String,
        model: Option<String>,
        serial: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            product,
            model,
            serial,
        }
    }
}

#[derive(Deserialize)]
pub struct AddDevice {
    name: String,
    product: String,
    model: Option<String>,
    serial: Option<String>,
}

impl Command for AddDevice {
    fn execute(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO device (name, product, model, serial)
            VALUES (?1, ?2, ?3, ?4)",
            (&self.name, &self.product, &self.model, &self.serial),
        )?;
        Ok(())
    }
}

impl AddDevice {
    pub fn new(
        name: String,
        product: String,
        model: Option<String>,
        serial: Option<String>,
    ) -> Self {
        Self {
            name,
            product,
            model,
            serial,
        }
    }
}

pub struct SwapDevice {
    id: u64,
    product: String,
    model: Option<String>,
    serial: Option<String>,
}

impl Command for SwapDevice {
    fn execute(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE device
            SET product = ?1, model = ?2, serial = ?3
            WHERE id = ?4",
            (&self.product, &self.model, &self.serial, self.id),
        )?;
        Ok(())
    }
}

impl SwapDevice {
    pub fn new(id: u64, product: String, model: Option<String>, serial: Option<String>) -> Self {
        Self {
            id,
            product,
            model,
            serial,
        }
    }
}

pub struct ReidentifyDevice {
    id: u64,
    model: String,
    serial: String,
}

impl Command for ReidentifyDevice {
    fn execute(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE device
            SET model = ?1, serial = ?2
            WHERE id = ?3",
            (&self.model, &self.serial, self.id),
        )?;
        Ok(())
    }
}

impl ReidentifyDevice {
    pub fn new(id: u64, model: String, serial: String) -> Self {
        Self { id, model, serial }
    }
}

pub struct ChangeDeviceFriendlyName {
    id: u64,
    name: String,
}

impl Command for ChangeDeviceFriendlyName {
    fn execute(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE device SET name = ?1 WHERE id = ?2",
            (&self.name, self.id),
        )?;
        Ok(())
    }
}

impl ChangeDeviceFriendlyName {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

pub struct RemoveDevice {
    id: u64,
}

impl Command for RemoveDevice {
    fn execute(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM device WHERE id = ?1", (self.id,))?;
        Ok(())
    }
}

impl RemoveDevice {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
