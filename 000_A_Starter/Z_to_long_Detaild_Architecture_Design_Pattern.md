
Yes. I understand what you want to build here.
This is **not a normal “design a Rust project” exercise**. We are going to turn backend architecture into a structured game where you progressively design a production system **before writing a single line of code**.
The important part is that I should **not dump the entire architecture on you at once**. You should have to make decisions, discover problems, revise designs, and gradually assemble the system.
I would call the game:
# 🏗️ Rust Backend Architecture Game
The goal is to train you to think like a backend/system architect who happens to implement systems in Rust.

---

# 1. The fundamental rules

Every project follows these rules.

### Rule 1 — No implementation code

We can discuss:

* Rust types
* structs
* enums
* traits
* `impl` blocks
* functions
* ownership
* borrowing
* `Arc`
* `Mutex`
* `RwLock`
* channels
* lifetimes
* async boundaries
* database schemas
* APIs
* protocols
* workflows

…but **we don't write the actual Rust implementation**.

You should eventually be able to take the architecture and implement it yourself.

---

### Rule 2 — Production-grade thinking

We don't design:

> "A library system with `Book`, `User`, and `borrow_book()`."

We design something closer to:

> "If 10,000 users are using this system concurrently, the database goes down temporarily, requests are retried, two users attempt to borrow the same copy simultaneously, an event needs to be published, and we need observability and auditability — what happens?"

We consider:

* concurrency
* failures
* consistency
* transactions
* idempotency
* retries
* validation
* authentication
* authorization
* persistence
* caching
* queues
* events
* observability
* configuration
* graceful shutdown
* error handling
* testing
* migrations
* deployment concerns
* security
* scalability

Only when relevant to the particular system.

---

# 2. The most important rule: we don't assume everything needs everything

For example, I will **not** blindly say:

> "Every struct gets `Arc<RwLock<T>>`."

Instead, we ask:

> Why does this data need shared ownership?

> Who owns it?

> Who mutates it?

> Does it actually need synchronization?

> Can ownership be transferred instead?

> Can the data live behind a database instead?

> Is `Arc` appropriate?

> Is `Mutex` appropriate?

> Is `RwLock` actually better?

This is extremely important for learning Rust architecture rather than memorizing patterns.

The same applies to traits.

We will **not create pointless traits for every struct**.

For every trait we introduce, we will ask:

> Why does this abstraction exist?

Possible reasons include:

* dependency inversion
* mocking
* multiple implementations
* domain behavior
* infrastructure isolation
* polymorphism
* framework requirements
* async abstraction
* testability

If a trait has no architectural reason to exist, we don't create it.

---

# 3. The game has stages

The stages are deliberately arranged approximately in the order a real architect would reason about a system.

The number of stages isn't fixed.

A tiny system might take **8 stages**.

A serious distributed system might take **25+ stages**.

We stop adding stages when the architecture is sufficiently understood.

---

# 🎮 Stage 0 — Game Setup

First we establish:

* system name
* system purpose
* users
* business context
* expected scale
* important constraints
* learning priorities
* complexity level

Then I give you the **mission briefing**.

Example:

> You are designing the backend of a production library-management platform used by 50,000 users.

But I won't immediately give you the architecture.

---

# 🎮 Stage 1 — Requirements Discovery

We establish what the system actually needs to do.

We separate:

### Functional requirements

What the system does.

Example:

* register users
* search books
* borrow books
* return books
* reserve books
* calculate fines

### Non-functional requirements

How the system must behave.

Example:

* availability
* latency
* consistency
* durability
* security
* scalability

### Explicitly out of scope

This is extremely important.

Real architecture requires knowing what **not** to build.

---

# 🎮 Stage 2 — Actors & Use Cases

We identify every important actor.

For example:

```text
Member
Librarian
Administrator
System
Notification Service
Payment Provider
```

Then map their interactions.

We'll identify:

* commands
* queries
* workflows
* permissions
* external interactions

This gives us the first meaningful view of the system.

---

# 🎮 Stage 3 — Domain Modeling

Now we start thinking like Rust programmers.

We discover:

* entities
* value objects
* aggregates
* domain concepts
* invariants
* state transitions

Then we decide which concepts deserve:

* `struct`
* `enum`
* newtype
* collection
* identifier type

For every important type we discuss:

* fields
* field types
* ownership
* optionality
* invariants
* visibility
* lifecycle

For example, instead of casually saying:

```text
User.id = String
```

we might decide that a production system deserves a dedicated:

```text
UserId
```

concept.

But **you will make/defend those decisions**, rather than me just handing them to you.

---

# 🎮 Stage 4 — Domain Rules & State Machines

Now we identify what can and cannot happen.

For example:

```text
Available
    ↓
Borrowed
    ↓
Returned
```

But perhaps:

```text
Reserved → Borrowed
Reserved → Cancelled
Borrowed → Overdue
```

We identify:

* legal transitions
* illegal transitions
* invariants
* business rules
* validation boundaries

This is where domain modeling becomes real.

---

# 🎮 Stage 5 — System Boundaries

Now we divide the system.

Potential boundaries might include:

```text
API
Application
Domain
Infrastructure
Persistence
Messaging
External integrations
```

But we don't assume a particular architecture.

We discuss alternatives such as:

* layered architecture
* modular monolith
* hexagonal architecture
* clean architecture
* CQRS
* event-driven architecture
* microservices

Then decide what is appropriate **for this particular system**.

---

# 🎮 Stage 6 — Component Architecture

Now we design the actual components.

For every component:

### Purpose

Why does it exist?

### Responsibilities

What does it own?

### Dependencies

What does it depend upon?

### Inputs

What comes into it?

### Outputs

What leaves it?

### Failure modes

What can go wrong?

### Ownership

Who owns the relevant state?

This gives us the architectural map.

---

# 🎮 Stage 7 — Rust Type Architecture

Now we go deep into Rust.

For every important type we document:

### Structs

* name
* fields
* field types
* ownership
* visibility
* invariants

### Enums

* variants
* why an enum is appropriate
* state representation
* serialization concerns

### Newtypes

* why a primitive isn't sufficient
* validation
* type safety

### Collections

* `Vec`
* `HashMap`
* `HashSet`
* `BTreeMap`
* etc.

and **why** each is appropriate.

---

# 🎮 Stage 8 — Ownership & Smart Pointer Architecture

This is a dedicated stage.

We explicitly inspect where Rust ownership becomes architecturally important.

For every shared object we ask:

```text
Who owns this?
Who reads it?
Who mutates it?
How long does it live?
Is it shared?
Is it concurrent?
```

Then decide whether we need:

* owned values
* references
* `Box<T>`
* `Arc<T>`
* `Rc<T>`
* `Mutex<T>`
* `RwLock<T>`
* atomics
* channels
* interior mutability

And importantly:

### Sometimes the answer is "none."

That's also a valid architectural decision.

---

# 🎮 Stage 9 — Traits & Abstractions

Now we identify every trait that genuinely deserves to exist.

For each trait:

```text
Trait
 ├── purpose
 ├── required behavior
 ├── associated types
 ├── methods
 ├── implementors
 └── architectural reason
```

For every implementation:

```text
Struct → Trait
```

we explain **why that implementation exists**.

Examples might include:

* repository abstractions
* service abstractions
* domain behavior
* authentication providers
* notification providers
* storage backends

But again:

> No abstraction without a reason.

---

# 🎮 Stage 10 — `impl` Blocks & Public API

Now we determine what each type actually knows how to do.

For every important struct we list:

### Constructors

* `new`
* builders
* factory concepts

### Domain methods

What the object can legitimately do.

### Query methods

What information it exposes.

### State-changing methods

What mutations are allowed.

### Conversion methods

Where appropriate.

### Validation methods

Where appropriate.

We distinguish:

```text
Domain behavior
Application orchestration
Infrastructure behavior
```

rather than putting everything into one giant service.

---

# 🎮 Stage 11 — Application Use Cases

Now we construct the actual workflows.

For every use case:

```text
Request
 ↓
Validation
 ↓
Authorization
 ↓
Application service
 ↓
Domain operation
 ↓
Persistence
 ↓
Events
 ↓
Response
```

We document the exact chronological flow.

For example:

> Borrow book

might become:

```text
HTTP request
→ authentication
→ authorization
→ validate request
→ load member
→ load book copy
→ verify availability
→ begin transaction
→ modify domain state
→ persist
→ create audit event
→ commit
→ publish event
→ response
```

And then we ask:

> What if step 7 fails?

That question is where real architecture starts becoming interesting.

---

# 🎮 Stage 12 — Persistence Architecture

Now we design the database side.

We discuss:

* database choice
* tables
* columns
* primary keys
* foreign keys
* indexes
* unique constraints
* check constraints
* relationships
* transactions
* isolation
* locking
* migrations
* connection pools

We map:

```text
Rust domain model
        ↕
Persistence model
        ↕
Database
```

and decide whether those models should actually be identical.

---

# 🎮 Stage 13 — Repository / Data Access Design

Now we determine:

* repositories
* queries
* commands
* transactions
* connection handling
* mapping
* error propagation

We also discuss where database-specific details are allowed to leak.

---

# 🎮 Stage 14 — API Architecture

If the project exposes an API, we design:

* endpoints
* HTTP methods
* request types
* response types
* status codes
* validation
* pagination
* filtering
* sorting
* error responses
* versioning

We distinguish:

```text
API DTO
≠
Domain Entity
≠
Database Model
```

when that separation is useful.

---

# 🎮 Stage 15 — Error Architecture

This gets its own stage because Rust makes error architecture particularly interesting.

We identify:

### Domain errors

Business rule failures.

### Application errors

Workflow failures.

### Infrastructure errors

Database/network/etc.

### API errors

What clients actually receive.

Then design the error conversion chain.

For example:

```text
DatabaseError
      ↓
RepositoryError
      ↓
ApplicationError
      ↓
ApiError
      ↓
HTTP response
```

We also discuss:

* recoverable errors
* retryable errors
* permanent errors
* user-facing errors
* internal errors
* logging without leaking sensitive information

---

# 🎮 Stage 16 — Concurrency & Race Conditions

This is one of the most important stages for Rust backend architecture.

We intentionally attack our design.

Examples:

> Two users borrow the same book simultaneously.

> Two workers process the same job.

> A request times out after the database commits.

> A message is delivered twice.

> Two administrators update the same resource.

Then determine:

* locking
* transactions
* optimistic concurrency
* pessimistic concurrency
* idempotency
* atomic operations
* unique constraints
* queues
* deduplication

This stage turns a "diagram" into an actual system design.

---

# 🎮 Stage 17 — Async Architecture

We determine where asynchronous execution is appropriate.

Discuss:

* Tokio
* tasks
* channels
* async traits
* cancellation
* timeouts
* backpressure
* worker pools
* graceful shutdown

And importantly:

> Where should we **not** use async?

---

# 🎮 Stage 18 — Events & Background Processing

If the system needs them, we design:

* domain events
* integration events
* queues
* workers
* scheduled jobs
* retries
* dead-letter handling
* idempotency

We trace:

```text
Command
 ↓
State change
 ↓
Event
 ↓
Queue
 ↓
Worker
 ↓
External action
```

---

# 🎮 Stage 19 — Authentication & Authorization

Depending on the project:

* identity
* sessions
* JWT
* OAuth
* API keys
* roles
* permissions
* resource-level authorization
* service authentication

We explicitly separate:

```text
Authentication
"Who are you?"

Authorization
"What are you allowed to do?"
```

---

# 🎮 Stage 20 — Observability

Production systems need to tell us what they're doing.

We design:

* structured logging
* tracing
* metrics
* request IDs
* correlation IDs
* health checks
* readiness
* liveness
* audit logs

And decide what should and shouldn't be logged.

---

# 🎮 Stage 21 — Configuration & Runtime

We determine:

* configuration structure
* environment variables
* secrets
* configuration validation
* development configuration
* production configuration
* feature flags if needed
* startup sequence
* shutdown sequence

---

# 🎮 Stage 22 — Security Review

We attack our own architecture.

Potential issues:

* injection
* authentication bypass
* authorization bugs
* secret leakage
* insecure serialization
* malicious input
* replay attacks
* rate abuse
* privilege escalation
* sensitive logging
* dependency vulnerabilities

The exact threats depend on the system.

---

# 🎮 Stage 23 — Testing Architecture

This is not simply:

> "Write unit tests."

We design the testing pyramid.

### Unit tests

Domain logic.

### Integration tests

Database/components.

### API tests

External behavior.

### End-to-end tests

Whole workflows.

### Property-based tests

Using **proptest**.

### Test fixtures

### Test doubles

### Testcontainers / equivalent infrastructure

when appropriate.

And as you specifically requested, the standard testing crate list will always include:

* `proptest`
* `test-case`
* `pretty_assertions`

Even if a particular project doesn't end up needing all three heavily, they remain in the project's testing-tool evaluation.

---

# 🎮 Stage 24 — Failure Injection

Now we deliberately break things.

Examples:

```text
Database unavailable
Queue unavailable
External API timeout
Duplicate event
Malformed message
Concurrent request
Worker crash
Process restart
Partial transaction
Network timeout
```

Then we determine whether the architecture behaves correctly.

---

# 🎮 Stage 25 — Performance & Scalability

We ask:

> What happens when usage becomes 10× larger?

Then:

* identify bottlenecks
* database bottlenecks
* indexes
* caching
* connection pools
* queue throughput
* CPU
* memory
* I/O
* horizontal scaling
* partitioning/sharding if justified

We don't prematurely distribute the system.

---

# 🎮 Stage 26 — External Crates

Only after understanding the architecture do we decide dependencies.

For each crate:

```text
crate
purpose
where used
why this crate
alternatives
why selected
production concerns
```

Typical categories might include:

```text
Async runtime
Web framework
Serialization
Database
Connection pooling
Error handling
Logging/tracing
Configuration
Authentication
Messaging
UUID
Time/date
Validation
Testing
Property testing
```

The exact crate selection depends on the project.

---

# 🎮 Stage 27 — Complete Architecture Review

At this point I stop teaching and become the reviewer.

I'll attack your architecture.

I'll ask things like:

> Why is this `Arc` necessary?

> Why does this service own the repository?

> What prevents duplicate borrowing?

> What happens if this event is published twice?

> Why is this trait needed?

> Why is this database constraint not represented in the domain?

> What happens after the request times out?

> Can this transaction partially succeed?

> What happens if the worker crashes here?

You defend the architecture.

If your reasoning is wrong, we redesign it.

---

# 🎮 Stage 28 — Final Architecture Blueprint

Only at the end do we assemble everything into one master specification.

It contains:

```text
System
├── Requirements
├── Actors
├── Use cases
├── Domain model
│   ├── structs
│   ├── enums
│   ├── newtypes
│   └── invariants
├── Ownership model
├── Smart pointers
├── Traits
├── impl blocks
├── Functions
├── Components
├── Workflows
├── Database
├── API
├── Errors
├── Concurrency
├── Async
├── Events
├── Security
├── Observability
├── Testing
├── Dependencies
├── Failure handling
└── Deployment architecture
```

Then we produce a **chronological implementation roadmap**.

---

# 🧠 But there's one more important part: YOU have to play

I don't want this to become:

> You ask → ChatGPT gives architecture → you read it → next project.

That would teach you very little.

Instead, each stage can have a **challenge**.

For example:

### Me

> We have 50,000 members and each physical book can only be borrowed by one member at a time.

> Design the core domain objects.

### You

You propose:

```text
Book
Member
Loan
```

### Me

I review your design.

Maybe I say:

> Good, but you have conflated a book title with a physical book copy.

Then we discuss why.

You redesign it.

Only then do we continue.

That means you're actually learning to **architect**, rather than learning to recognize architecture diagrams.

---

# 🧩 Three levels of information

Throughout the game, I'll keep three categories separate.

### 🟢 Known

Decisions we've already established.

### 🟡 Proposed

Something we are considering but haven't committed to.

### 🔴 Unresolved

Something that still needs an architectural decision.

This prevents us from accidentally treating an early idea as a final architecture.

---

# 📜 Architecture Decision Records

Whenever we make an important decision, we'll record it conceptually as an ADR:

```text
Decision:
Use PostgreSQL.

Why:
...

Alternatives:
...

Rejected because:
...

Consequences:
...
```

This is extremely useful because you're learning **why architectures are shaped the way they are**, not just what they look like.

---

# 🧪 Testing follows the architecture

Another important rule:

We don't design the whole system and then randomly say:

> "Now add tests."

Instead, every component gets a testing strategy **when we design it**.

For example:

```text
Domain
→ unit tests
→ property tests

Repository
→ integration tests

API
→ API/integration tests

Worker
→ workflow tests

Whole system
→ E2E tests
```

Then we define the **chronological order in which you would build and test the actual system**.

---

# 🏁 The final learning objective

By the end of a project, you should be able to look at a blank repository and think:

```text
What are the boundaries?

What owns what?

What is the domain?

What can change?

What cannot change?

Where are the invariants?

Where does concurrency exist?

Where does persistence happen?

Where should async exist?

Where do failures occur?

Where do traits actually help?

Where does ownership become shared?

Where should the database enforce rules?

Where should Rust enforce rules?

What happens when something fails?

How do I test this?

How does this behave at 10× scale?
```

**That is the actual game.**

---

# 🚀 The reusable game-initiation prompt

From now on, you can start a completely new architecture game by giving me **just the system name**.

For example:

> `Library Management System`

Everything else is encoded in this prompt.

# Rust Backend Architecture Game — Initialize Project

We are going to play a structured learning game for designing a **production-grade backend system in Rust**.

The system I want to design is:

**SYSTEM NAME:** `<SYSTEM NAME>`

---

## Core Rules

1. **No implementation code.**
   We may discuss Rust syntax conceptually, but do not write the actual implementation.

2. Design the system as if it were intended for real production use.

3. Do not simplify the architecture merely to make it easier.

4. Do not introduce unnecessary complexity merely to make it look production-grade.

5. Every architectural decision must have a reason.

6. Never introduce a trait, smart pointer, abstraction, component, database mechanism, queue, cache, or service without explaining why it exists.

7. Do not assume that every struct needs a trait.

8. Do not assume that every shared object needs `Arc`, `Mutex`, or `RwLock`.

9. Explicitly explain ownership and concurrency decisions.

10. When a Rust feature is relevant, explain both:

* how it fits into the architecture
* why it is preferable to the alternatives

11. We are learning architecture, not copy-pasting a finished project.

12. I should be required to make architectural decisions whenever practical.

---

# Learning Priority

The primary goal is to teach me **backend systems design and architecture through Rust**.

Prioritize:

1. System decomposition
2. Domain modeling
3. Architecture boundaries
4. Ownership
5. Concurrency
6. Async architecture
7. Database architecture
8. API architecture
9. Error architecture
10. Traits and abstractions
11. Production failure handling
12. Testing architecture
13. Observability
14. Security
15. Scalability

Rust should be used as a tool for understanding these architectural concepts, not merely as a programming language exercise.

---

# What We Must Eventually Design

For the selected system, eventually cover all applicable areas:

## 1. Requirements

* functional requirements
* non-functional requirements
* constraints
* assumptions
* scale
* explicit out-of-scope features

## 2. Actors & Use Cases

Identify:

* users
* administrators
* external systems
* workers
* services
* commands
* queries
* important workflows

## 3. Domain Model

Identify:

* entities
* value objects
* aggregates
* domain concepts
* identifiers
* newtypes
* structs
* enums
* collections
* invariants
* state machines
* valid/invalid transitions

For every important struct:

* fields
* field types
* ownership
* visibility
* invariants
* lifecycle

For every enum:

* variants
* purpose
* state representation
* serialization implications where relevant

## 4. Ownership & Memory Architecture

For every important shared or long-lived object, determine:

* owner
* consumers
* mutation requirements
* lifetime
* whether sharing is required
* whether synchronization is required

Evaluate where appropriate:

* owned values
* references
* `Box<T>`
* `Arc<T>`
* `Rc<T>`
* `Mutex<T>`
* `RwLock<T>`
* atomics
* channels
* interior mutability

Explicitly explain why each is used or why it is deliberately NOT used.

## 5. Traits

Identify every trait that genuinely provides architectural value.

For each:

* trait name
* purpose
* methods
* associated types if applicable
* implementors
* reason abstraction is useful
* alternatives
* why those alternatives were rejected

Do not create artificial traits.

## 6. impl Blocks & Functions

For every important type, identify conceptually:

* constructors
* factory methods
* domain methods
* state-changing methods
* query methods
* validation methods
* conversion methods
* helper methods

For every function:

* purpose
* inputs
* outputs
* ownership/borrowing considerations
* errors
* side effects
* component boundary

Separate:

* domain behavior
* application orchestration
* infrastructure behavior

## 7. Components

Design every required component.

For each component document:

* purpose
* responsibilities
* owned state
* dependencies
* inputs
* outputs
* failure modes
* concurrency model
* testing strategy

## 8. Application Workflows

For every important use case, provide the chronological flow.

Example format:

Request
→ validation
→ authentication
→ authorization
→ application service
→ domain operation
→ persistence
→ event
→ external action
→ response

Also explain failure paths.

## 9. Persistence

Design:

* database choice
* tables
* columns
* relationships
* primary keys
* foreign keys
* unique constraints
* check constraints
* indexes
* transactions
* isolation
* locking
* migrations
* connection pools

Explain the relationship between:

* domain models
* persistence models
* database schema

## 10. Repository/Data Access

Design:

* repositories
* queries
* commands
* transactions
* mapping
* connection handling
* error boundaries

## 11. API

If applicable, design:

* endpoints
* HTTP methods
* request types
* response types
* DTOs
* validation
* status codes
* pagination
* filtering
* sorting
* errors
* versioning

## 12. Error Architecture

Design the complete error flow.

Distinguish:

* domain errors
* application errors
* infrastructure errors
* API errors
* retryable errors
* permanent errors
* internal errors
* client-visible errors

Explain error conversions.

## 13. Concurrency

Identify all race conditions and concurrent workflows.

Explicitly investigate:

* duplicate operations
* conflicting updates
* concurrent state transitions
* race conditions
* transaction conflicts
* worker duplication
* idempotency
* locking
* optimistic concurrency
* pessimistic concurrency

## 14. Async

Where applicable, design:

* Tokio runtime
* tasks
* channels
* async boundaries
* cancellation
* timeouts
* backpressure
* worker pools
* graceful shutdown

Also explicitly identify where async is unnecessary.

## 15. Events & Background Jobs

If applicable:

* domain events
* integration events
* queues
* workers
* scheduled jobs
* retries
* dead-letter handling
* deduplication
* idempotency

## 16. Authentication & Authorization

If applicable:

* authentication
* authorization
* sessions
* JWT
* OAuth
* API keys
* roles
* permissions
* resource-level authorization

Keep authentication and authorization conceptually separate.

## 17. Observability

Design:

* structured logging
* tracing
* metrics
* request IDs
* correlation IDs
* health checks
* readiness
* liveness
* audit logging

## 18. Configuration & Runtime

Design:

* configuration
* environment variables
* secrets
* validation
* startup
* shutdown
* development configuration
* production configuration

## 19. Security

Perform a project-specific security review covering relevant threats such as:

* injection
* authentication bypass
* authorization failures
* secret leakage
* malicious input
* replay
* abuse
* privilege escalation
* insecure serialization
* sensitive logging
* dependency vulnerabilities

## 20. Testing Architecture

Always include these testing crates in the dependency evaluation:

* `proptest`
* `test-case`
* `pretty_assertions`

Then determine all other appropriate testing tools.

Design:

* unit tests
* integration tests
* API tests
* end-to-end tests
* property-based tests
* fixtures
* test doubles
* database tests
* concurrency tests
* failure tests

Do not merely list tests. Explain what each test validates.

## 21. Failure Engineering

Deliberately break the architecture.

Consider applicable failures such as:

* database unavailable
* database timeout
* queue unavailable
* external API failure
* duplicate messages
* malformed messages
* worker crash
* process restart
* partial failure
* transaction failure
* network timeout
* request timeout after successful processing

Explain the expected behavior.

## 22. Performance & Scalability

Evaluate:

* CPU
* memory
* database
* indexes
* connection pools
* caching
* queues
* I/O
* throughput
* latency
* horizontal scaling
* partitioning/sharding where justified

Do not introduce distributed architecture without justification.

## 23. External Crates

Provide the eventual crate evaluation.

For every crate:

* crate
* purpose
* component using it
* why it was selected
* alternatives
* trade-offs
* production considerations

Always include:

* `proptest`
* `test-case`
* `pretty_assertions`

in the testing dependency evaluation.

---

# Game Structure

Do NOT reveal the entire final architecture at the beginning.

Break the project into stages.

The number of stages should depend on the system complexity.

A simple project may have around 8–15 stages.

A complex production system may require 20–30+ stages.

Use approximately this progression:

1. Mission / project setup
2. Requirements discovery
3. Actors and use cases
4. Domain modeling
5. Domain rules and state machines
6. System boundaries
7. Component architecture
8. Rust type architecture
9. Ownership and smart pointers
10. Traits and abstractions
11. impl blocks and functions
12. Application workflows
13. Persistence
14. Repository/data access
15. API
16. Error architecture
17. Concurrency
18. Async architecture
19. Events/background processing
20. Authentication/authorization
21. Observability
22. Configuration/runtime
23. Security
24. Testing
25. Failure engineering
26. Performance/scalability
27. Dependency/crate review
28. Architecture review
29. Final architecture blueprint
30. Chronological implementation roadmap

Add, remove, merge, or split stages when the actual system requires it.

Do not force irrelevant architecture into a project.

---

# Interactive Game Rules

This must be an interactive learning game.

Do not simply give me the answer.

At appropriate stages:

1. Explain the problem.
2. Give me the architectural context I need.
3. Ask me to make a decision.
4. Let me propose the design.
5. Critique my decision.
6. Explain what I got right.
7. Explain what is problematic.
8. Challenge my assumptions.
9. Let me revise the architecture.
10. Record the final decision.

When I make an architectural decision, distinguish:

* Known
* Proposed
* Unresolved

Do not treat a proposal as final until we agree on it.

---

# Architecture Decision Records

For important decisions, maintain conceptual ADRs containing:

* decision
* context
* alternatives
* selected approach
* reasons
* rejected alternatives
* consequences

---

# No-Code Constraint

Do not provide implementation code unless I explicitly change the rules later.

Pseudo-code may only be used when absolutely necessary to explain a workflow, and even then prefer structured prose or diagrams.

The purpose is to make me capable of implementing the system myself after the architecture is complete.

---

# Chronological Build Plan

At the end, produce the actual implementation order.

For example:

```text
Foundation
→ Domain types
→ Domain rules
→ Persistence
→ Repositories
→ Application services
→ API
→ Authentication
→ Events
→ Workers
→ Observability
→ Testing
→ Failure handling
→ Integration
```

But determine the correct order specifically for this project.

For every implementation phase explain:

* what is built
* why it is built now
* dependencies
* what should be tested
* what must already work
* what milestone proves it works

---

# Small Milestones

Break the final implementation roadmap into small meaningful milestones.

Each milestone should produce something conceptually testable.

Avoid giant milestones such as:

> "Build backend."

Prefer:

> "Domain can represent a valid loan lifecycle."

or:

> "Repository can atomically prevent two users from acquiring the same resource."

---

# Final Deliverable

At the end of the game, produce a complete architecture specification containing:

* requirements
* assumptions
* actors
* use cases
* architecture style
* boundaries
* components
* structs
* enums
* newtypes
* fields
* ownership
* smart pointers
* traits
* trait implementations
* impl blocks
* functions
* helpers
* workflows
* database
* repositories
* APIs
* errors
* concurrency
* async
* events
* authentication
* authorization
* observability
* configuration
* security
* testing
* external crates
* failure handling
* scalability
* ADRs
* dependency graph
* component interaction
* chronological implementation order
* chronological testing order
* small milestones

The final result should be detailed enough that I could implement the system myself without needing you to invent the architecture while coding.

---

# Most Important Learning Rule

The goal is NOT:

> "Give me the correct architecture."

The goal is:

> **Teach me how to arrive at the correct architecture myself.**

Challenge me.

Make me justify decisions.

Point out hidden problems.

Introduce realistic production scenarios.

Ask what happens when things fail.

Force me to think about ownership, concurrency, consistency, and boundaries.

Do not rescue me too early.

If my design is wrong, let me discover why through targeted questions before revealing the solution.

---

# Start the Game

The only information I am intentionally providing right now is the system name:

**<SYSTEM NAME>**

Start with **Stage 0 — Mission Briefing**.

Do not jump ahead to the final architecture.

Give me the initial scenario, constraints, expected scale, learning objectives, and first architectural challenge.

That prompt is the **master game launcher**. From the next project onward, you only need to replace `<SYSTEM NAME>`.

And I particularly like the distinction between **learning the architecture vs. receiving an architecture** here. That should make projects like a library system, URL shortener, payment system, chat backend, camera-management backend, job queue, etc. progressively harder while teaching the same underlying architectural thinking.
