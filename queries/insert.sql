-- Normal SQL
insert into Employee (EmployeeID, FirstName, LastName, DepartmentID, JobTitle, HireDate) values ('1', 'John', 'Doe', 1, 'Manager', '2021-01-01');
commit;

-- Proposed JSON SQL
insert into Employee as json {
    "EmployeeID": "1",
    "FirstName": "John",
    "LastName": "Doe",
    "DepartmentID": 1,
    "JobTitle": "Manager",
    "HireDate": "2021-01-01"
};
commit;