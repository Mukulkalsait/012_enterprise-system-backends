
# Rust Backend Architecture Game

We are going to design and architect this system together:

**SYSTEM:** `<SYSTEM NAME>`

This is an interactive **Rust backend architecture game**, not a coding exercise.

## Rules

* No implementation code.
* Only discuss Rust syntax when it helps understand an architectural decision.
* I must design things myself first.
* Do not immediately give me the solution.
* Give me enough context to make a decision, then ask me to design it.
* Review my answer.
* Correct mistakes.
* Identify missing pieces.
* Explain why my decisions are good or bad.
* Let me revise my design when necessary.
* Once a piece is correct, consider it locked and move forward.

## Core Design Areas

Whenever we design a component, cover the relevant parts:

* structs
* fields
* Rust types
* enums
* newtypes
* relationships
* ownership
* borrowing
* smart pointers
* `Arc`, `Rc`, `Box`, `Mutex`, `RwLock`, etc. when relevant
* traits
* trait implementations
* `impl` blocks
* methods
* helper functions
* validation
* errors
* external crates
* testing strategy
* `proptest`
* `test-case`
* `pretty_assertions`
* component-specific tests

Do NOT force irrelevant concepts into the design.

## Game Stages

### Stage 1 — System Skeleton

Give me the basic requirements and ask me to identify the major components/entities of the system.

I design them.

You review and correct them.

Do not design everything for me.

---

### Stage 2 — Component Design

Take one component at a time.

For each component, go through this cycle:

1. Explain the component's responsibility.
2. Ask me to design its structs/types/fields.
3. Review my design.
4. Let me correct it.
5. Decide ownership and relationships.
6. Decide smart-pointer usage.
7. Decide enums/newtypes where needed.
8. Design its `impl` block.
9. Design its main functions/methods.
10. Identify helper functions.
11. Identify necessary traits and trait implementations.
12. Identify external crates.
13. Design its tests.
14. Lock the component.
15. Move to the next component.

Do not move to the next component until the current one is reasonably complete.

---

### Stage 3 — Special Architecture

If the project requires something that does not naturally fit into the component cycle, introduce a temporary special stage.

Examples:

* folder/module architecture
* database architecture
* event system
* message queue
* connector
* authentication
* caching
* networking
* serialization
* configuration
* background workers
* concurrency architecture
* project-specific infrastructure

Only introduce these when the actual project requires them.

---

### Stage 4 — Higher-Level System

After the major components are complete, design the higher-level system that coordinates them.

For example:

```text
Library
├── books
├── users
├── loans
└── ...
```

Again follow the same cycle:

* structs
* fields
* types
* ownership
* smart pointers
* traits
* impl
* functions
* helpers
* crates
* tests

---

### Stage 5 — System Workflows

Connect the components through the important real-world workflows.

For every major workflow:

1. I propose the flow.
2. You review it.
3. You identify missing steps.
4. We examine ownership and responsibility.
5. We examine failures and edge cases.
6. I revise the workflow.
7. We lock it.

Pay particular attention to:

* validation
* business rules
* concurrency
* consistency
* errors
* ownership
* component boundaries

---

### Stage 6 — Final Review

Once the system is assembled, perform a concise architecture review.

Check:

* missing components
* incorrect responsibilities
* unnecessary abstractions
* incorrect ownership
* unnecessary smart pointers
* missing traits
* missing methods
* missing error handling
* missing tests
* important concurrency problems
* project-specific missing infrastructure
* important external dependencies

Then declare the architecture complete.

## Important Learning Principle

The purpose is to teach me how to **think about backend architecture in Rust**.

Do not optimize for giving me the fastest correct answer.

Optimize for making me reason about the answer myself.

Challenge my decisions.

Ask questions such as:

* "Why does this need to be shared?"
* "Who owns this?"
* "Why is this a `Vec` instead of IDs?"
* "Should this rule live here?"
* "Does this actually need a trait?"
* "What happens if two requests do this simultaneously?"
* "What happens if this operation fails halfway?"
* "Should this be a domain concern or infrastructure concern?"

But don't overwhelm me with questions. Keep the game moving.

## Complexity / Duration

The goal is approximately:

* small system: 30–60 minutes
* medium system: 1–2 hours
* complex system: 2–4 hours

Do not turn the exercise into a massive architecture document.
Keep each stage focused and interactive.
If the system is too large, naturally split it into manageable components rather than expanding the process unnecessarily.

## Start

Start with **Stage 1 — System Skeleton**.
First give me only the essential scenario, requirements, constraints, and context I need.
Then ask me to identify the major components myself.
Do not reveal the complete architecture upfront.



AND YES MOST IMPORTANT, instead of giving 1 stage give me all the 6 stages at once. I will store them in local file and remind you on time to time about the stage we are on and when to move. 
