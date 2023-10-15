use rusqlite::{Connection, Error, Row};

pub struct DeviceRestAddressUpdate {
    pub protocol: String,
    pub address: String,
}

pub struct DeviceRestCredentialsUpdate {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct DeviceRest {
    pub id: u64,
    pub device_id: u64,
    pub protocol: String,
    pub address: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub metric_collection: bool
}

impl From<&Row<'_>> for DeviceRest {
    fn from(row: &Row<'_>) -> Self {
        DeviceRest {
            id: row.get(0).expect("Couldn't cast id column to u64"),
            device_id: row.get(1).expect("Couldn't cast device_id column to u64"),
            protocol: row.get(2).expect("Couldn't cast protocol column to String"),
            address: row.get(3).expect("Couldn't cast address column to String"),
            username: row.get(4).expect("Couldn't cast username column to String"),
            password: row.get(5).expect("Couldn't cast password column to String"),
            metric_collection: row.get(6).expect("Couldn't cast metric_collection column to bool"),
        }
    }
}

pub fn add_address(
    conn: &Connection,
    device_id: u64,
    address: &DeviceRestAddressUpdate,
) -> Result<DeviceRest, Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO device_rest (device_id, protocol, address)
        VALUES (?1, ?2, ?3)
        RETURNING id, device_id, protocol, address, username, password, metric_collection",
    )?;
    stmt.query_row((&device_id, &address.protocol, &address.address), |row| {
        Ok(DeviceRest::from(row))
    })
}

pub fn select(conn: &Connection, device_id: u64) -> Result<DeviceRest, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, protocol, address, username, password, metric_collection
        FROM device_rest
        WHERE id = ?1",
    )?;
    stmt.query_row((device_id,), |row| Ok(DeviceRest::from(row)))
}

pub fn update_credentials(
    conn: &Connection,
    device_id: u64,
    credentials: &DeviceRestCredentialsUpdate,
) -> Result<DeviceRest, Error> {
    let mut stmt = conn.prepare(
        "UPDATE device_rest
        SET username = ?1, password = ?2
        WHERE id = ?3
        RETURNING id, device_id, protocol, address, username, password, metric_collection",
    )?;
    stmt.query_row(
        (&credentials.username, &credentials.password, &device_id),
        |row| Ok(DeviceRest::from(row)),
    )
}

pub fn toggle_collection(
    conn: &Connection,
    device_id: u64,
    collection_status: bool,
) -> Result<DeviceRest, Error> {
    let mut stmt = conn.prepare(
        "UPDATE device_rest
        SET metric_collection = ?1
        WHERE id = ?2
        RETURNING id, device_id, protocol, address, username, password, metric_collection",
    )?;
    stmt.query_row((&collection_status, &device_id), |row| {
        Ok(DeviceRest::from(row))
    })
}
