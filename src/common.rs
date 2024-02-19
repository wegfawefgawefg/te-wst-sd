const BIG_SMALL_THRESHOLD: u32 = 32 * 1024 * 1024; // 32MB
const TOO_BIG_THRESHOLD: u32 = 256 * 1024 * 1024; // 256MB

type UserId = u32;
type FileSizeInBytes = u32;
type SubmissionId = u32;
type SubmissionPartId = u32;
type WebSocket = u32;

enum BigOrSmall {
    Big,
    Small,
}
