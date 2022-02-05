CREATE TABLE "Student" (
  "StudentId" INT NOT NULL,
  "ParentId" INT NOT NULL,
  "Name" VARCHAR(30) NOT NULL,
  "Age" INT NOT NULL,
  "Address" VARCHAR(25) NOT NULL,
  "Phone" VARCHAR(20) NOT NULL,
  CONSTRAINT "PK_Student" PRIMARY KEY ("StudentId")
);

CREATE TABLE "Parent" (
  "ParentId" INT NOT NULL,
  "StudentId" INT NOT NULL,
  "PartnerId" INT NOT NULL,
  "Name" VARCHAR(30) NOT NULL,
  "Address" VARCHAR(25) NOT NULL,
  "Phone" VARCHAR(20) NOT NULL,
  CONSTRAINT "PK_Parent" PRIMARY KEY ("ParentId")
);

ALTER TABLE "Student" ADD CONSTRAINT "FK_StudentParentId"
  FOREIGN KEY ("ParentId") REFERENCES "Parent" ("ParentId");

ALTER TABLE "Parent" ADD CONSTRAINT "FK_ParentStudentId"
  FOREIGN KEY ("StudentId") REFERENCES "Student" ("StudentId");

ALTER TABLE "Parent" ADD CONSTRAINT "FK_ParentPartnerId"
  FOREIGN KEY ("PartnerId") REFERENCES "Parent" ("ParentId");
