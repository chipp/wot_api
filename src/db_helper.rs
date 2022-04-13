use rusqlite::{params, Connection};

pub struct DbHelper {
    conn: Connection,
}

impl DbHelper {
    pub fn new() -> DbHelper {
        let db_path = std::env::var("DB_PATH").unwrap_or("data.sqlite".to_string());

        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT, 
                title TEXT NOT NULL, 
                count TEXT, 
                completed BOOLEAN
            );",
            [],
        )
        .unwrap();

        DbHelper { conn }
    }
}

impl DbHelper {
    pub fn get_all_items(&self) -> Vec<Item> {
        let mut select = self
            .conn
            .prepare("SELECT id, title, count, completed FROM items ORDER BY id ASC")
            .unwrap();

        select
            .query_map([], |row| {
                Ok(Item {
                    id: row.get::<_, u16>(0)?,
                    title: row.get::<_, String>(1)?,
                    count: row.get::<_, Option<String>>(2)?,
                    completed: row.get::<_, bool>(3)?,
                })
            })
            .unwrap()
            .map(Result::unwrap)
            .collect()
    }

    pub fn add_new_item<'j>(&self, title: &'j str, count: Option<&'j str>) -> Item {
        let mut insert = self
            .conn
            .prepare("INSERT INTO items (title, count, completed) VALUES (?, ?, false)")
            .unwrap();

        insert.execute(params![title, count]).unwrap();

        Item {
            id: self.conn.last_insert_rowid() as u16,
            title: title.to_string(),
            count: count.map(String::from),
            completed: false,
        }
    }

    pub fn remove_item(&self, id: u16) {
        let mut insert = self.conn.prepare("DELETE FROM items WHERE id = ?").unwrap();

        insert.execute(params![id]).unwrap();
    }
}

pub struct Item {
    pub id: u16,
    pub title: String,
    pub count: Option<String>,
    pub completed: bool,
}
