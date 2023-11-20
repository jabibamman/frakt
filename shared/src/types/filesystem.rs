/// Represents the types of directories used in the application.
///
/// `Current` - Refers to the current working directory.
/// `Workspace` - Designates the workspace directory, often used in development environments.
pub enum DirType {
    Current,
    Workspace,
}

/// Specifies the types of files or entities in a file system.
///
/// `File` - Represents a standard file.
/// `Directory` - Represents a directory.
pub enum FileType {
    File,
    Directory,
}

/// Enumerates the supported image file extensions.
///
/// `PNG` - Portable Network Graphics format, used for images with transparency.
/// `JPG` - JPEG compression format, commonly used for digital photography.
/// `JPEG` - A variant of the JPG extension, often used interchangeably.
pub enum FileExtension {
    PNG,
    JPG,
    JPEG,
}
