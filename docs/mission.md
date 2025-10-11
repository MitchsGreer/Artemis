# Mission

The mission of the project is to create a tool that will allow teams to manage requirements in a way that is simple, trackable, and flexible.

## Use Cases

1. A user has agreed to some set of requirements with their customer and must now track the development of their application to ensure those requirements are satisfied.
1. A user has agreed to some set of requirements with their customer and must add some new requirements for subsystems that must be satisfied for the success of the system.
1. A user needs to be able to trace a requirements through its dependencies.
1. A user needs to be able to show that they have test results for all agreed upon requirements.
1. A user needs to print a report of all the requirements, their dependencies, and if they have been tested to success.
1. A user has requirements that they must satisfy from some standard. This standard is lives in a PDF and they must somehow enter it into this application.

## General Ideas

The general idea would be that a user has a document made up of blocks of text. Each of the blocks of text are not all
necessarily requirements, but they may be. This document should be able to be rendered. The current idea is to save it
in markdown format. This would allow it to be diff-able as well.This document would be housed in a hierarchial folder
structure that would allow the user to sort documents into folders. This folder structure would be stored in some kind
of repository, the current idea is to use git. Sections in these documents must be able to link to each other to show
dependencies. These links need to be exportable to a traceability format to show what requirements depend on each other.
It would be ideal to be able to link requirements to test results as well to show that they have passed or failed.
