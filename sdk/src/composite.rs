use crate::cmd_disk;
use crate::Client;

/// Represents operations that are made up of multiple API calls and may also
/// access the local file system
pub trait ClientCompositeExt {
    /// Create a disk from a file upload
    /// Composite operation consisting of multiple API calls
    /// Arguments
    /// - `description`: Human readable description of the disk
    /// - `disk`: Name of the disk to create
    /// - `disk_block_size`: The desired size of the disk's blocks
    /// - `disk_size`: The size of the disk to create. If unspecified, the size
    /// of the file will be used, rounded up to the nearest GB
    /// - `image`: If supplied, create an image with the given name. Requires
    /// the creation of a snapshot.
    /// - `image_description`: The description of the image created out of the
    /// snapshot of this disk
    /// - `image_os`: The OS of this image (e.g. Debian)
    /// - `image_version`: The The version of this image (e.g. 11, focal)
    /// - `path`: Path to the file to import
    /// - `project`: Name or ID of the project
    /// - `snapshot`: If supplied, create a snapshot with the given name.
    /// - `extra_out`: Print progress and error messages
    ///
    fn disk_import(&self) -> cmd_disk::builder::DiskImport;
}

impl ClientCompositeExt for Client {
    fn disk_import(&self) -> cmd_disk::builder::DiskImport {
        cmd_disk::builder::DiskImport::new(self)
    }
}
