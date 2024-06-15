//！ csv reader for conf/wiki.csv
//！ tokio: update 1 mins/once
use crate::tools::{Storage, StoragePool, Wiki};

async fn read_csv() -> Option<Vec<Wiki>> {
    let mut rdr = csv::Reader::from_path("conf/wiki.csv").unwrap();
    let mut wikis = Vec::new();
    for wiki in rdr.deserialize() {
        let wiki: Wiki = wiki.unwrap();
        wikis.push(wiki);
    }
    Some(wikis)
}

pub async fn wiki_update(pool: StoragePool) {
    let mut storage = Storage;
    tokio::spawn(async move {
        loop {
            let wikis = read_csv().await.unwrap();
            for wiki in wikis {
                storage.insert_wiki(&pool, wiki).await.unwrap();
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });
}
