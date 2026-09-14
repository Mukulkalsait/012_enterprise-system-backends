```rust
// ---------- IDs ----------

type UserId = uuid::Uuid;
type StudentId = uuid::Uuid;
type TeacherId = uuid::Uuid;
type CourseId = uuid::Uuid;
type ClassId = uuid::Uuid;
type SubjectId = uuid::Uuid;
type ExamId = uuid::Uuid;
type PaperId = uuid::Uuid;
type ExamPaperId = uuid::Uuid;
type AcademicSessionId = uuid::Uuid;
type CenterID: uuid::Uuid;


// ---------- User ----------


enum UserType { Student, Teacher, Staff, Admin, }

// ## `addresses`
//
// Because address is a separate concept:
//
// | id     | user_id  | address_line_1 | address_line_2 | city   | state | country | postal_code |
// | ------ | -------- | -------------- | -------------- | ------ | ----- | ------- | ----------- |
// | addr_1 | user_101 | 12 Main Street | NULL           | Bhopal | MP    | India   | 462001      |
//
// If we guarantee exactly one address per user, we can make `user_id` unique.
struct Address{
  address_line_1: String,
  address_line_2: Option<String>,

  city: String,
  state: String,
  country: String,
  postal_code: String, // Y: or External create.
}

enum DocumentType{ Aadhar, Passport, DrivingLicense, VoterId, BirthCertificate, TenthMarkSheet, TwelfthMarkSheet, Domicile, TransferCertificate, Other }

// ## `documents`
//
// `Vec<Documents>` becomes:
//
// | id    | user_id  | document_type  | document_no | url         | created_at | updated_at |
// | ----- | -------- | -------------- | ----------- | ----------- | ---------- | ---------- |
// | doc_1 | user_101 | Aadhar         | XXXX1234    | /docs/doc_1 | 2026-08-01 | 2026-08-01 |
// | doc_2 | user_101 | TenthMarkSheet | MARK123     | /docs/doc_2 | 2026-08-01 | 2026-08-01 |
struct Documents{
  document_type: DocumentType,
  document_no: String,
  url: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}


// ## `contact_numbers`
//
// Your `Vec<ContactNo>` becomes rows:
//
// | id      | user_id  | country_code | no         | default | is_verified |
// | ------- | -------- | ------------ | ---------- | ------- | ----------- |
// | phone_1 | user_101 | 91           | 9876543210 | true    | true        |
// | phone_2 | user_101 | 91           | 9123456789 | false   | false       |
//
// This is much better than storing a JSON array.
struct ContactNo{
  country_code: string, // Y: or External create.
  no: String,
  default: bool,
  is_verified: bool,
}

// ## `emergency_contacts`
//
// And:
//
// | id          | user_id  | name          | relationship |
// | ----------- | -------- | ------------- | ------------ |
// | emergency_1 | user_101 | Kavita Sharma | Mother       |
//
// Then the emergency contact's phone can be:
//
// | id      | emergency_contact_id | country_code | no         | default | is_verified |
// | ------- | -------------------- | ------------ | ---------- | ------- | ----------- |
// | phone_3 | emergency_1          | 91           | 9876543211 | true    | false       |
//
struct EmergencyContact{
  name: String,
  relationgship: String,
  contact: ContatNo,
}

enum SignUpType{ Email, Google, ... }

// ## `users`
//
// Authentication/account-level information:
//
// | id       | type    | email                     | email_verified | signup_type | external_name | external_id | hashed_pass | created_at | updated_at |
// | -------- | ------- | ------------------------- | -------------- | ----------- | ------------- | ----------- | ----------- | ---------- | ---------- |
// | user_101 | Student | [a@x.com](mailto:a@x.com) | true           | Google      | Rahul         | google_123  | NULL        | 2026-08-01 | 2026-08-01 |
// | user_201 | Teacher | [b@x.com](mailto:b@x.com) | true           | Email       | NULL          | NULL        | hash_xyz    | 2026-08-02 | 2026-08-02 |
//
// User
//  ├── UserInfo
//  ├── Address
//  ├── ContactNumbers
//  ├── EmergencyContacts
//  │      └── ContactNumber
//  └── Documents
struct User {
  id: UserId,
  type: UserType,

  email: String,
  email_verified:bool,

  signup_type: SignUpType, 
  external_name: Option<String>, // eg. google name 
  external_id: Option<String>,

  hashed_pass: String,

  user_info: Option<UserInfo>,

  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>
}

// ## `user_infos`
//
// One-to-one with `users`:
//
// | user_id  | first_name | middle_name | last_name | gender | nationality | dob        |
// | -------- | ---------- | ----------- | --------- | ------ | ----------- | ---------- |
// | user_101 | Rahul      | NULL        | Sharma    | Male   | Indian      | 2001-05-12 |
// | user_201 | Priya      | NULL        | Singh     | Female | Indian      | 1988-03-21 |
//
// `user_id` is both the relationship to `users` and effectively the identity of this information record.
// `UserInfo` is optional, so a newly-created Google account can exist here without having completed their personal information.
// users
//   │
//   └── user_infos
//       │
//       ├── contact_numbers
//       ├── emergency_contacts
//       │       └── contact_numbers
//       ├── addresses
//       └── documents
//
// This is the key idea:
//
// > **One domain object does NOT have to equal one database table.**
//
// Our Rust model can be convenient:
struct UserInfo{

  first_name: String,
  middle_name: Option<String>,
  last_name: String,
  gender: Gender,
  nationality: String,
  profile_photo: String,

  phone: Vec<ContactNo>,

  emergency_contact: EmergencyContact,

  address: Address,
  documents: Vec<Documents>,

  dob: NaiveDate,
}

enum Gender{ Male, Female, Other, }

// ---------- Student ----------



enum StudentStatus { Active, Inactive, Graduated, Transferred, Suspended, Expelled, Withdrawn, }

// DB: students
// Permanent student record; session-specific academic placement lives elsewhere.
// `user_id` connects the student record to the common `users` table.
//
// | id        | user_id  | enrolment_id | admission_id | status    | course_id | created_at | updated_at |
// | --------- | -------- | ------------ | ------------ | --------- | --------- | ---------- | ---------- |
// | student_1 | user_101 | ENR001       | ADM001       | Active    | XW24S      | 2026-08-01 | 2026-08-01 |
// | student_2 | user_102 | ENR002       | ADM002       | Active    | XW24S      | 2026-08-02 | 2026-08-02 |
// | student_3 | user_103 | ENR003       | NULL         | Graduated | ION@3      | 2026-08-03 | 2026-08-03 |
//
struct Student {
  id: StudentId,
  user_id: UserId,

  enrolment_id: String,
  addmission_id: Option<String>,
  status: StudentStatus,
  course_id: CourseId,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>
}



// ---------- Teacher ----------

enum TeacherStatus { Active, OnLeave, Suspended, Transferred, Resigned, Retired, Inactive,}

enum TeacherLevel { Instructor, SeniorInstructor, SeniorFaculty, }

// ### `teachers`
// Again, `user_id` connects the teacher to the common `users` table.
//
// | id        | user_id  | employee_id | status  | level         | created_at | updated_at |
// | --------- | -------- | ----------- | ------- | ------------- | ---------- | ---------- |
// | teacher_1 | user_201 | EMP001      | Active  | SeniorFaculty | 2026-08-01 | 2026-08-01 |
// | teacher_2 | user_202 | EMP002      | OnLeave | Instructor    | 2026-08-02 | 2026-08-02 |
struct Teacher {
  id: TeacherId,
  user_id: UserId,
  employee_id: String,

  status: TeacherStatus,
  level: TeacherLevel,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>
}


// ---------- Course ----------

enum CourceDuration {
  QuadSemister, HexaSemister, OctaSemister
}


// For `Course`, the `subjects: Vec<SubjectId>` becomes a **relationship table**, just like `Class → Students`.
//
// ### `courses`
//
// | id    | name            | description                      | university_id | duration     | created_at | updated_at |
// | ----  | --------------- | -------------------------------- | ------------- | ------------ | ---------- | ---------- |
// | XW24S | MSc Mathematics | Master of Science in Mathematics | M401          | HexaSemester | 2026-08-01 | 2026-08-01 |
// | ION@3 | MSc Physics     | Master of Science in Physics     | P345          | HexaSemester | 2026-08-01 | 2026-08-01 |
//
// ### `course_subjects`
//
// | course_id | subject_id |
// | --------- | ---------- |
// | XW24S     | subject_1  |
// | XW24S     | subject_2  |
// | XW24S     | subject_3  |
// | ION@3     | subject_2  |
// | ION@3     | subject_4  |
//
// So:
//
// while `Physics` can simultaneously belong to `ION@3`.
// **That's why `Vec<SubjectId>` is useful in the Rust model, while the database uses `course_subjects`.**

struct Course {
  id: CourseId,
  name: String,
  description:String,
  
  universty_id: String,

  duration: CourceDuration,
  subjects: Vec<SubjectId>,

  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>
}


// ---------- Subject ----------

// DB: subjects
// Reusable subjects; one subject can be referenced by multiple courses.
// 
// | id        | name        | description          | created_at       | updated_at       |
// | --------- | ----------- | -------------------- | ---------------- | ---------------- |
// | subject_1 | Mathematics | Advanced mathematics | 2026-08-01 10:00 | 2026-08-01 10:00 |
// | subject_2 | Physics     | Fundamental physics  | 2026-08-01 10:05 | 2026-08-01 10:05 |
struct Subject {
  id: SubjectId,
  name: String,
  description: String,

  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>
}


// ---------- Class ----------

/// DB: classes
/// ~20–40 rows per academic session for a typical college.
/// Each row represents one cohort/group for a course in a session.
/// ## Relationship Table
/// When two entities have a relationship such as `Class → Students`,we normally don't store `Vec<StudentId>` in one database column. 
/// Instead, we create a separate table such as `class_students` containing `class_id` and `student_id`.
/// 
/// IMP:  Basicaly it means billow students: Vec<StudentId> will create sepearte table with table classID = which will contain all list of student_id. 
/// imagein a 2D table of class with last coll is 3D going in Z-axis contaiing all the ids 
///  - faster finding,
///  - easy CRUD operationos, 
///  - adds small extra code 

// --------------------------------------------------------------------------
// | class_id | course_id       | academic_session_id | description           |
// | -------- | --------------- | ------------------- | --------------------- |
// | class_a  | course_msc_math | 2026_27             | Mathematics Section A |
// --------------------------
// | class_id | student_id |
// | -------- | ---------- |
// | class_a  | student_1  |
// | class_a  | student_2  |
// | class_a  | student_3  |

struct Class {
  class_id: ClassId,
  course_id: CourseId,
  academic_session_id: AcademicSessionId,
  students: Vec<StudentId>, 
  description: String,
}

// ---------- Teaching Assignment ----------

// | teacher_id | subject_id | class_id | academic_session_id |
// |------------|------------|----------|---------------------|
// | teacher_1  | subject_1  | class_a  | 2026_27             |
struct TeachingAssignment {
  teacher_id: TeacherId,
  subject_id: SubjectId,
  class_id: ClassId,
  academic_session_id: AcademicSessionId,
}

// ---------- Academic Session ----------

// | id           | year    | description           |
// | ------------ | ------- | --------------------- |
// | session_2026 | 2026_27 | Academic year 2026–27 |
// | session_2027 | 2027_28 | Academic year 2027–28 |
struct AcademicSession {
  academic_session_id: AcademicSessionId,
  year: String,
  description: String,
}

enum AcademicSemister{ First, Second, Third, Forth, Fifth, Sixth, Seventh, Eighth, }
enum StudentAcademicStatus{ Promoted, Failed, Passed, Detained, }

// onlu use is to know which student is in which class of which year,
// | student_id | academic_session_id | class_id | semester |
// | ---------- | ------------------- | -------- | -------- |
// | student_1  | session_2026        | class_a  | Third    |
// | student_2  | session_2026        | class_a  | Third    |
// | student_1  | session_2027        | class_b  | Fourth   |
struct StudentAcademicRecord {
  student_id: StudentID,
  academic_session_id: AcademicSessionID,
  class_id : ClassID,
  semister: AcademicSemister,
  status: StudentAcademicStatus,
}



// ---------- Attendance ----------


enum AttendanceStatus{ Present, Absent, Late, Leave, HalfDay }


// | id    | user_id | academic_session_id | date       | status  |
// | ----- | ------- | ------------------- | ---------- | ------- |
// | att_1 | user_1  | session_2026        | 2026-08-01 | Present |
// | att_2 | user_1  | session_2026        | 2026-08-02 | Absent  |
// | att_3 | user_2  | session_2026        | 2026-08-01 | Late    |
struct Attendance {
  user_id: UserId,
  academic_session_id: AcademicSessionId,
  date: NaiveDate,
  status: AttendanceStatus,
  recorded_by: UserId,
  marked_at: DateTime<Utc>,
  source: String, //mac id
}


// ---------- Exam ----------

enum ExamType{ Internal, Midterm, Final, Supplementory, Entrance, Practical, Other, }

// | id     | academic_session_id | name            | exam_type | start_date | end_date   |
// | ------ | ------------------- | --------------- | --------- | ---------- | ---------- |
// | exam_1 | session_2026        | June 2026 Final | Final     | 2026-06-01 | 2026-06-20 |
struct Exam {
  exam_id: ExamId,
  academic_session_id: AcademicSessionId,
  name: String,
  exam_type: ExamType,

  start_date: NaiveDate, 
  end_date: NaiveDate,
}


// ---------- Paper ----------

type CenterID: uuid::Uuid;

// | id   | exam_id | subject_id | date_time        | duration | center_id | total_marks | passing_percentage | minimum_marks |
// | ---- | ------- | ---------- | ---------------- | -------: | --------- | ----------: | -----------------: | ------------: |
// | ep_1 | exam_1  | math       | 2026-06-05 10:00 |      180 | center_1  |         100 |                 40 |            40 |
// | ep_2 | exam_1  | physics    | 2026-06-08 10:00 |      180 | center_1  |         100 |                 40 |            40 |
struct ExamPaper {
  paper_id: ExamPaperId,

  exam_id: ExamId,
  subject_id: SubjectId, 

  date_time: DateTime<Utc>,
  duration: u8,
  centerid: CenterID,

  total_marks: u16,
  passing_percentage: Option<u8>,
  minimum_marks: u16,
}



// more repetative informaiton.
//
// | id      | exam_paper_id | student_id | obtained_marks | is_cheated |
// | ------- | ------------- | ---------- | -------------: | ---------- |
// | paper_1 | ep_1          | student_1  |             78 | false      |
// | paper_2 | ep_1          | student_2  |             65 | false      |
// | paper_3 | ep_1          | student_3  |             31 | true       |
struct Paper{
  id: PaperID,
  exam_paper_id: ExamPaperId,

  student_id: StudentId, // perticular for per student
  invigilator_id: Vec<TeacherId>, // invigilator's wise

  obtained_marks: Option<u16>,
  is_cheated: bool,
}




// ---------- Student Result ----------
// Derived object — not stored as a source-of-truth entity.

/// Y: NO table needed.
struct StudentResult {
  student_id: StudentId,
  exam_id: ExamId,
  overall_result: StudentAcademicStatus,
}

```
