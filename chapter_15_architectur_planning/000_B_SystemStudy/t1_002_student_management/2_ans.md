
so we will be needing, 

tip: all the ID will be uuid: v4
and ... mean there will be more general fields which we will decide later so dont point that out that you forget to add name, created_at etc etc on this stage. 


## some combine student teacher designs:

enum UserType {student teacher}
struct UserInfo{ UserType: id : studentid / teacherid ... }
struct Attendence { UserType: id : studentid / teacherid ... }


## then cources: 
type courceID: 
struct Cource{ courceID , subjects: Vec<subjects> , classID }

##  classes

type classID: 
struct Class{classID, classteacher: teacherID, courceID, ...}

## subject

type sujbectID:
struct subject{ sujbectID, hod_teacher: teacherID, sbjectID,...  }

## then student, containing: 

type studentID, 
enum studentStatus {Active, Inactive, Transfered, etc}
struct student{ including studentID, studentStatus, courceID, ...  }


## TEacher: 

type teacherID:
enum teacherStatus: { Active, OnProjectOutOfStaion, LongLeave, Inactive, Transfered, etc}
enum teacherLevel: { HOD, MainInstructor, SubjectTeacher, etc}
struct Teacher{ teacherID, teacherStatus, teacherLevel, ... }

## exams:

> 1 student - 1 paper
type paperID: 
struct paper{paperID, studentID, subjectID, invisilator: teacherID, marks, ...}

> 1 student -> all paper
type studentResultID:
struct StudentResult{studentResultID, sujbectID, papers: Vec<paper>, }

> 1 class -> all student -> all papers
type classResultID:
struct ClassResult{ courceID, classID, results: Vec<studentResult>, ... }

> all classes + session
type examID: 
struct exam{ examID, year, session, class_based_result: Vec<ClassResult> , ...}

ok i designed this part but i feel like if we want to collect data from deep we need to do iterator over iterator which will be really slow, if i am not worong? waht better way can we impliment? hashmaps? i hvent studied hashpamping yet... should i study it beforee movign aheade? 
what is the best way to impoiiment it? 

## Academic year:

type AcademicYearsessionID: 
struct AcademicYearSession{ AcademicYearsessionID, year, session, 
teachers: Vec<Teachers> ,
students: Vec<Students>,
cources: Vec<Cources>
classes: Vec<Classes>
subjects: Vec<Subject>
exams: Vec<Exams>
...
}











