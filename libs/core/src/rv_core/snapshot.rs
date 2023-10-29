pub trait Snapshotable {
    type Snapshot;

    fn snapshot(&self) -> Self::Snapshot;
}
