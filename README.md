# What is Amble?

Amble is an omnitool for your data, collecting information into a unified
format, and providing powerful interfaces to program, view and manipulate
that information.

Amble is designed to work well for individuals and small businesses that want a 
central way to manage fragmented data. One that provides convenience without
sacrificing their control, or their privacy. That lends itself to power users 
and non-technical users.

# Roadmap

1. *(current)* Neovim interface for writing and searching org mode notes, 
   alongside facilities to serialize those notes into a sqlite database
   from Amble intermediate representation (AIR).
2. Simple fold evaluation (no lisp integration)
3. Lisp integration into fold evaluation, alongside providing
   support for user defined lisp functions that can be uesd during
   fold evaluation
4. Python and Javascript/Typescript syntax for constructing folds, 
   alongside general Python bindings for working with Amble data.
5. A JSON parser and renderer for AIR
6. A web library for developing UIs on top of Amble data

## Project Goals

Amble was motivated by the need for a better way to integrate the notes you take on your computer
with the rest of your information. Power users often have trouble with current
software offerings that fragment their data across apps and services. Amble first aims to provide a plugin for
Neovim that allows users to interact with their Amble data in Markdown and Org mode, 
making Amble a natural alternative to existing note taking/knowledge management systems.

Amble's super powers are in its pattern matching system, allowing you to take any structure in your data, and transform
it into any other structure. Have TODOS spread across your notes? Pattern match them into a single list of TODOS. 
Want to view and edit your TODOS on the web? Take that list of todos, ask Amble to convert them to JSON, and send
them off to a web server. When that web server sends back an updated JSON of TODOS, Amble will take that and update
all of your TODOS across your data accordingly.

Amble's core is compiled as a shared library, and aims to provide bindings for a variety of popular languages. Giving
you full access to its powerful systems for manipulating your data. Amble also aims to provide a cli to make it easy
to import information from files across a variety of popular formats.

### Amble Org Mode Interface

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
```

### Not just for notes

Amble has a goal of being able to manage and organize large amounts of data. Performantly calculating folds,
and searches in milliseconds. Down the road, we hope to provide tools for developers to easily have their
amble system integrate data from many different sources, not just from their handwritten notes. Amble should
work with any data that can be serialized into a tree of nodes with properties.
