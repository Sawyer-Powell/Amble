# What is Amble?

Amble is an omnitool for your data, collecting information into a unified
format, and providing powerful interfaces to program, view and manipulate
that information.

Amble is being developed first as a Neovim plugin and cli tool, as a format for
taking notes, and aggregating information. This will help formulate the core
of Amble, which is being developed as a shared library.

Development will then move onto fleshing Amble out as an alternative document
store, providing facilities for users to build web applications on top of
their Amble data.

# Roadmap

1. **(current)** Neovim interface for writing and searching org mode notes, 
   alongside facilities to serialize those notes into a sqlite database
   from Amble intermediate representation (AIR).
   1. Org mode parser **(done)**
   2. Serialize into sqlite **(done)**
   3. De-serialize from sqlite **(done)**
   4. Lua interface to Amble shared library **(done)**
   5. Search, save, and edit functionalities for Neovim **(current)**
2. Simple fold evaluation (no lisp integration)
3. Propagate data updates backwards through system when editing the results of folds
4. Lisp integration into fold evaluation, alongside providing
   support for user defined lisp functions that can be used during
   fold evaluation
5. A JSON parser and renderer for AIR
6. A web library for developing UIs on top of Amble data
7. Other parsers/renderers for Markdown, XML, etc.
8. Bindings for other languages, like Python, to efficiently script Amble

## Project Goals

Amble was motivated by the need for a better way to integrate the notes you take on your computer
with the rest of your information. Power users often have trouble with current
software offerings that fragment their data across apps and services. The core of amble is AIR, the 
Amble Intermediate Representation. Amble is designed to take in data from a variety of formats, like
org mode, markdown, JSON, xml/html, and transform that data into AIR. Once that data has been transformed, Amble
is able to leverage its pattern matcher to allow you to query and manipulate that data efficiently.

Because AIR is compatible with org mode, and markdown, you can interact with, and edit, your data
like you would edit your notes. Amble first aims to provide a plugin for
Neovim that allows users to interact with their Amble data in Markdown and Org mode, 
making Amble a natural alternative to existing note taking/knowledge management systems.

Amble then aims to provide a web framework that can allow users to create UIs for manipulating and viewing
their data.

## Amble Org Mode and Pattern Matching

Like standard org mode, you can have headlines, todos,
and assign properties to those headlines.
```org
* TODO Design homepage layout
  :Project: Website Redesign
  :Due: <2023-10-14>

* TODO Organize team-building event
  :Project: HR Activities
  :Due: <2023-10-30>

* TODO Need to do the laundry
  :Due: <2023-09-20>

* Redesign Sprint Meeting 2024-08-17
  * UI Feedback
    Scott really like the design of the arrows, but would like
    more contrast in the button borders.

    * TODO Should sit down with Katie to scope out the upcoming management screen
      :Project: Website Redesign
      :Due: <2024-08-25>
      Make sure to think through how users are going to be onboarded

  * API Feedback
    Stacy was having performance issues pulling reports from the finance page
 ... 
```

Elsewhere in your notes, you can define a *fold*, which matches certain patterns in your notes,
and allows you to collect those matches in a new list.

```org
* FOLD
  * FROM
    * "TODO" todo-title
      :Project: "Website Redesign"
      :Due: due-date
  * TO
    * "TODO" todo-title
      :Due: due-date
* TODO Design homepage layout
  :Due: <2023-10-14>
* TODO Should sit down with Katie to scope out the upcoming management screen
  :Due: <2024-08-25>
```
If you edit "Design homepage layout" to "Design layout for homepage", both the data in the fold
result and the original data that result was pulled from, will be updated. **Folds allow you to both view your data, and efficiently update your data at its source.**

You can do more advanced transformations through the use of an embedded Lisp interpreter,
and use the `[]` syntax for capturing data into groups

```org
* FOLD
  * FROM
    * "TODO" todo-title
      :Project: project
      :Due: due-date
  * TO
    * [(if-nil project "Uncategorized")]
      * todo-title "- due:" (date-to-string due-date)
        (if (> (today) due-date) "Overdue!" "Still Upcoming")
* HR Activities
  * Organize team-building event - due: October 30th, 2023
    Overdue!
* Uncategorized
  * Need to do the laundry - due: September, 20th, 2023
    Overdue!
* Website Redesign
  * Design homepage layout - due: October 14th, 2023
    Overdue!
  * Should sit down with Katie to scope out the upcoming management screen - due: August 25th, 2024
    Still Upcoming
