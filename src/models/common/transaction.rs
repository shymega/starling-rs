use uuid::Uuid as UUID;

#[derive(Clone, Debug, Copy)]
pub struct Transaction {
    pub feed_item_uid: UUID,
    pub category_uid: UUID,
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            feed_item_uid: UUID::new_v4(),
            category_uid: UUID::new_v4(),
        }
    }
}
