# Data Definition Language (DDL) Specification

## Overview

This document specifies the syntax and conventions for defining data models in our custom Data Definition Language (DDL). Each model represents a table in the database, and each field within a model represents a column within the table. The DDL supports basic data types, nullable fields, primary keys, and foreign key relationships.

## Syntax

Each model is defined within a `model` block, using curly braces `{}` to encapsulate its fields. A field is defined by specifying its name followed by its type. The language treats definitions line by line.

### Basic Syntax

- **Model Definition**: Models are defined with the `model` keyword followed by the model name and curly braces.

```plaintext
model ModelName {
    FieldName FieldType
}
```

- **Fields**: Each field within a model is defined on a new line, with the field name followed by its type.

```plaintext
FieldName FieldType
```

### Data Types

The DDL supports the following basic data types:
- `String`
- `Int`
- `Float`
- `Date`
- `Blob`

### Special Field Annotations

- **Nullable Fields**: Fields that can contain null values are marked with a `?` immediately after the type.

```plaintext
FieldName FieldType?
```

- **Primary Key**: Fields that are used as primary keys are annotated with `@id` immediately after the type.

```plaintext
FieldName FieldType @id
```

- **Foreign Key Relationships**: Fields that reference another model's primary key are annotated with `@references(ModelName, FieldName)` immediately after the type, even for nullable fields.

```plaintext
FieldName FieldType @references(ReferencedModel, ReferencedField)
```

## Examples

### Employee Model

Defines employees with basic information, including a nullable `DepartmentID` that references the `Department` model, and a nullable `JobTitle`, and a `HireDate`.

```plaintext
model Employee {
    EmployeeID String @id
    FirstName String
    LastName String
    DepartmentID Int? @references(Department, DepartmentID)
    JobTitle String?
    HireDate Date
}
```

### Department Model

Represents departments within the organization, identified by `DepartmentID`.

```plaintext
model Department {
    DepartmentID Int
    DepartmentName String
}
```

### Salary Model

Tracks salaries of employees over time, including an optional `ToDate` for historical records. `EmployeeID` serves as a foreign key referencing `Employee`.

```plaintext
model Salary {
    SalaryID Int
    EmployeeID Int @references(Employee, EmployeeID)
    Salary Float
    FromDate Date
    ToDate Date?
}
```

## Conclusion

This specification outlines the basic syntax and conventions for defining models using our custom DDL. It supports defining models, fields, primary keys, nullable fields, and establishing simple relationships between models through foreign keys. The inclusion of foreign key relationships, including nullable references, facilitates the representation of complex relational data structures in our database schema.