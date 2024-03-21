model Employee {
    EmployeeID String @id
    FirstName String
    LastName String 
    DepartmentID Int?
    JobTitle String? 
    HireDate Date 
}

model Department {
    DepartmentID Int 
    DepartmentName String
}

model Salary {
    SalaryID Int
    EmployeeID Int @references(Employee, EmployeeID)
    Salary Float
    FromDate Date
    ToDate Date? 
}
