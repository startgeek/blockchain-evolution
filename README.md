# Blockchain Evolution in Rust lesson 2 EVM based chains

## Overview
This repository serves as an **educational resource** for understanding **distributed systems, the Rust programming language, and the historical evolution of blockchain technology**. It is designed to provide a hands-on experience in building a simplified blockchain network with all steps in revolution. Nothing is better than **learning by doing**, and this project allows you to **practice real blockchain concepts in code**, ensuring a deep and practical understanding.

## Lesson 2: Ethereum's Virtual Machine (Simplified)

Building upon the foundational knowledge gained in **Lesson 1 (Bitcoin PoW - [see Master branch])**, this second lesson delves into the core of the Ethereum network: the **Ethereum Virtual Machine (EVM)**. While Lesson 1 focused on the fundamental blockchain structure and the Proof-of-Work consensus mechanism pioneered by Bitcoin, Lesson 2 shifts our focus to the concept of **state transition and smart contract execution**, which are central to Ethereum's innovation.

**In this part of Lesson 2, we will be creating a simplified model of the EVM.** This will involve implementing the basic components necessary to understand how Ethereum executes bytecode and manages state. We will focus on:

* **A simplified instruction set:** Defining a small set of fundamental operations that our mini-EVM can understand and execute.
* **A basic memory model:** Implementing a way for the EVM to store and retrieve data during execution.
* **A rudimentary stack:** Understanding how the EVM uses a stack-based architecture for computation. (this is very important concept which unfortnatley some seasoned programmers cant grasp fully resulting in all types of hacks and bugs)
* **State management:** Creating a simple mechanism to represent and update the state of our "blockchain" based on the execution of these instructions.

**The goal of this lesson is not to build a fully functional EVM, but rather to demystify its core principles and provide a practical understanding of how smart contracts are executed at a low level.** By implementing this simplified version, you will gain invaluable insights into the design choices and complexities involved in creating a virtual machine for decentralized applications.

### Learning Objectives for Lesson 2:

* Understand the fundamental role of the Ethereum Virtual Machine (EVM) in the Ethereum ecosystem.
* Grasp the concept of state transition as it relates to smart contract execution.
* Learn about the stack-based architecture of the EVM.
* Implement a simplified instruction set for a virtual machine.
* Create a basic memory model for data storage within the virtual machine.
* Develop a rudimentary mechanism for managing the state of the "blockchain".

### Structure of Lesson 2:

This lesson will likely be broken down into the following steps:

1.  **Introduction to the EVM:** A high-level overview of the Ethereum Virtual Machine, its purpose, and its key components.
2.  **Defining the Simplified Instruction Set:** We will define a small set of opcodes (instructions) that our mini-EVM will support. Examples might include basic arithmetic operations, memory access, and simple control flow.
3.  **Implementing the Memory Model:** We will create a data structure to represent the memory available to our simplified EVM.
4.  **Building the Execution Stack:** We will implement the stack data structure that the EVM uses for operand manipulation.
5.  **Creating the Core Execution Loop:** This will involve writing the logic that fetches instructions, decodes them, and executes them using the stack and memory.
6.  **Implementing Basic State Management:** We will create a simple way to store and update the "world state" based on the execution of our instructions.
7.  **Testing our Simplified EVM:** We will write basic "programs" (sequences of our simplified opcodes) to test the functionality of our mini-EVM.

### Getting Started with Lesson 2:

1.  **Ensure Rust is Installed:** If you haven't already, make sure you have Rust installed on your system. You can find installation instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Navigate to the Lesson 2 Directory:** If this repository is structured with separate directories for each lesson, navigate to the Ethereum-pow branch.
3.  **Follow the Implementation Steps:** The subsequent files and instructions in this directory will guide you through the process of building the simplified EVM.

### Further Exploration:

at this point this code is only builds. you would learn by reading the code in next session we would actually write smart contracts for our EVM compile it and run it.

By completing this lesson, you will gain a much deeper appreciation for the inner workings of the Ethereum Virtual Machine and the fundamental principles behind smart contract execution. Good luck, and happy coding!