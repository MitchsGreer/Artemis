# Requirement

This application tracks the connection of requirements to each other. This is
what a requirement should contain to be considered complete.

## Requirement Attributes

| Attribute    | Type        | Description                                                                                                        |
| :----------- | :---------- | :----------------------------------------------------------------------------------------------------------------- |
| UUID         | UUID or int | A unique identifier for this requirement in this set of requirements.                                              |
| ID           | string      | A requirement ID given by by the user to identify this requirement.                                                |
| Text         | string      | The text for this ID, this is where the user indicates what the requirement is, this should be in markdown format. |
| Dependencies | List[UUID]  | A list of the requirements/Documents that depend on this requirement. They might be derived from this one.         |
| Dependents   | List[UUID]  | A list of requirements/Documents that this requirement depends on.                                                 |
| Risks        | List[Risk]  | A list of risks associated to this requirement.                                                                    |

## Connection Example

```mermaid
flowchart LR

subgraph S1
    R1_S1
    R2_S1
    R3_S1
    R4_S1
end

subgraph S2
    R1_S2
    R2_S2
    R3_S2
    R4_S2
end

subgraph S3
    R1_S3
    R2_S3
    R3_S3
    R4_S3
end

R1_S1 --> S2;
R2_S1 --> R1_S3;
R2_S1 --> R2_S3;
R4_S1 --> R3_S3;
R3_S3 --> R4_S3;
```
