use uuid::Uuid as UUID;

#[derive(Clone, Debug, Copy)]
#[derive(Clone, Debug, Default)]
pub struct Transaction {
    pub feed_item_uid: UUID,
    pub category_uid: UUID,
}
}
