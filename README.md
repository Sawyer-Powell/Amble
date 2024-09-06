# What is Amble?

Amble is an omnitool for your data, collecting information into a unified
format, and providing powerful interfaces to program, view and manipulate
that information.

Amble is designed to work well for individuals and small businesses that want a 
central way to manage fragmented data. One that provides convenience without
sacrificing their control, or their privacy. That lends itself to power users 
and non-technical users.

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

### Amble Org Mode Neovim Interface

#### Note taking and Pattern Matching

Like standard org mode, you can have headlines, todos,
and assign properties to those headlines.
```org
* TODO Design homepage layout
  :Project: Website Redesign
  :Due: <2023-10-14>

* TODO Organize team-building event
  :Project: HR Activities
  :Due: <2023-10-30>

* Redesign Sprint Meeting 2024-08-17
 ** UI Feedback
    Scott really like the design of the arrows, but would like
    more contrast in the button borders.

  *** TODO Should sit down with Katie to scope out the upcoming management screen
      :Project: Website Redesign
      :Due: <2024-08-25>
      Make sure to think through how users are going to be onboarded

 ** API Feedback
    Stacy was having performance issues pulling reports from the finance page
 ... 
```
Elsewhere in your notes, you can define a *fold*, which matches certain patterns in your notes,
and allows you to collect those matches in a new list.
```org
* FOLD
 ** FROM
  *** "TODO" todo-title
      :Project: "Website Redesign"
      :Due: due-date
 ** TO
  *** "TODO" todo-title
      :Due: due-date
* TODO Design homepage layout
  :Due: <2023-10-14>
* TODO Should sit down with Katie to scope out the upcoming management screen
  :Due: <2024-08-25>
```

## Amble is designed for non-technical users

While use of Amble's core is suitable for highly technical people, it's flexibility allows us to build GUI systems
suitable for non-technical people. 

Once the core of Amble is stable, and provides value for power users, Amble aims to provide an extensible web interface,
in a similar vein to Notion, that will allow those without programming experience to still leverage Amble's powerful
internal systems.
