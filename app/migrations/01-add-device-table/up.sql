CREATE TABLE device(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    product TEXT NOT NULL,
    model TEXT,
    serial TEXT,
    UNIQUE(model, serial)
);

CREATE TABLE device_rest(
    id INTEGER PRIMARY KEY,
    device_id INTEGER NOT NULL UNIQUE,
    protocol TEXT NOT NULL DEFAULT 'http',
    address TEXT NOT NULL UNIQUE,
    username TEXT,
    password TEXT,
    metric_collection BOOLEAN DEFAULT FALSE NOT NULL,
    CONSTRAINT fk_device_rest_device
        FOREIGN KEY (device_id) REFERENCES device(id) ON DELETE CASCADE
)