# Sphinx & reStructuredText Documentation

## Overview

Sphinx is a documentation generator that converts reStructuredText (RST) into various output formats. This integration covers project documentation, API docs, and integration with the planning structure.

## Documentation Structure

```
docs/
├── conf.py                    # Sphinx configuration
├── index.rst                  # Documentation root
├── requirements.txt           # Docs dependencies
├── Makefile                   # Build commands
├── _static/                   # Static assets
│   ├── css/
│   │   └── custom.css
│   └── img/
├── _templates/                # Custom templates
├── api/                       # API reference
│   ├── index.rst
│   └── modules/
├── guides/                    # User guides
│   ├── index.rst
│   ├── getting-started.rst
│   ├── configuration.rst
│   └── deployment.rst
├── reference/                 # Reference documentation
│   ├── index.rst
│   ├── cli.rst
│   ├── configuration.rst
│   └── api-reference.rst
├── architecture/              # Architecture docs
│   ├── index.rst
│   ├── overview.rst
│   ├── components.rst
│   └── decisions/             # Architecture Decision Records
│       ├── index.rst
│       └── adr-001-database.rst
└── changelog.rst              # Version history
```

## Initial Setup

### conf.py

```python
# docs/conf.py
# Sphinx configuration file

import os
import sys
from datetime import datetime

# Add source path for autodoc
sys.path.insert(0, os.path.abspath('..'))

# -- Project information -----------------------------------------------------
project = 'My Project'
copyright = f'{datetime.now().year}, My Organization'
author = 'My Organization'
version = '1.0'
release = '1.0.0'

# -- General configuration ---------------------------------------------------
extensions = [
    'sphinx.ext.autodoc',           # Auto-generate from docstrings
    'sphinx.ext.autosummary',       # Summary tables
    'sphinx.ext.viewcode',          # Source code links
    'sphinx.ext.intersphinx',       # Link to other projects
    'sphinx.ext.napoleon',          # Google/NumPy style docstrings
    'sphinx.ext.todo',              # TODO directives
    'sphinx.ext.graphviz',          # GraphViz diagrams
    'sphinxcontrib.plantuml',       # PlantUML diagrams
    'sphinxcontrib.mermaid',        # Mermaid diagrams
    'sphinx_copybutton',            # Copy button for code blocks
    'sphinx_tabs.tabs',             # Tabbed content
    'myst_parser',                  # Markdown support
]

# Source file extensions
source_suffix = {
    '.rst': 'restructuredtext',
    '.md': 'markdown',
}

# Master doc
master_doc = 'index'

# Exclude patterns
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# Templates path
templates_path = ['_templates']

# -- Options for HTML output -------------------------------------------------
html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']
html_css_files = ['css/custom.css']

html_theme_options = {
    'navigation_depth': 4,
    'collapse_navigation': False,
    'sticky_navigation': True,
    'includehidden': True,
    'titles_only': False,
}

# Logo and favicon
# html_logo = '_static/img/logo.png'
# html_favicon = '_static/img/favicon.ico'

# -- Extension configuration -------------------------------------------------

# Autodoc
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'special-members': '__init__',
    'undoc-members': True,
    'exclude-members': '__weakref__',
}

# Napoleon
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = True

# Intersphinx
intersphinx_mapping = {
    'python': ('https://docs.python.org/3', None),
}

# PlantUML
plantuml = 'plantuml'
plantuml_output_format = 'svg'

# Mermaid
mermaid_version = '10.6.1'

# TODO
todo_include_todos = True

# Autosummary
autosummary_generate = True
```

### requirements.txt

```
# docs/requirements.txt
sphinx>=7.0
sphinx-rtd-theme>=2.0
sphinx-copybutton>=0.5
sphinx-tabs>=3.4
sphinxcontrib-plantuml>=0.27
sphinxcontrib-mermaid>=0.9
myst-parser>=2.0
```

### Makefile

```makefile
# docs/Makefile
SPHINXOPTS    ?=
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = .
BUILDDIR      = _build

.PHONY: help clean html linkcheck

help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

clean:
	rm -rf $(BUILDDIR)

html:
	@$(SPHINXBUILD) -b html "$(SOURCEDIR)" "$(BUILDDIR)/html" $(SPHINXOPTS) $(O)

linkcheck:
	@$(SPHINXBUILD) -b linkcheck "$(SOURCEDIR)" "$(BUILDDIR)/linkcheck" $(SPHINXOPTS) $(O)

serve: html
	python -m http.server --directory $(BUILDDIR)/html 8000

watch:
	sphinx-autobuild "$(SOURCEDIR)" "$(BUILDDIR)/html" $(SPHINXOPTS) $(O)
```

## RST Templates

### index.rst (Root)

```rst
.. My Project documentation master file

Welcome to My Project
=====================

.. toctree::
   :maxdepth: 2
   :caption: Getting Started

   guides/getting-started
   guides/configuration
   guides/deployment

.. toctree::
   :maxdepth: 2
   :caption: Reference

   reference/cli
   reference/configuration
   reference/api-reference

.. toctree::
   :maxdepth: 2
   :caption: Architecture

   architecture/overview
   architecture/components
   architecture/decisions/index

.. toctree::
   :maxdepth: 2
   :caption: API Documentation

   api/index

.. toctree::
   :maxdepth: 1
   :caption: About

   changelog

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
```

### Getting Started Guide

```rst
Getting Started
===============

This guide will help you get up and running with My Project.

Prerequisites
-------------

Before you begin, ensure you have:

* Python 3.10 or later
* Node.js 20 or later
* PostgreSQL 16

Installation
------------

.. tabs::

   .. tab:: pip

      .. code-block:: bash

         pip install myproject

   .. tab:: Nix

      .. code-block:: bash

         nix develop

   .. tab:: Container

      .. code-block:: bash

         podman run -it ghcr.io/myorg/myproject

Quick Start
-----------

1. Initialize the project:

   .. code-block:: bash

      myproject init

2. Configure your environment:

   .. code-block:: bash

      myproject config set database.url postgres://localhost:5432/myapp

3. Start the server:

   .. code-block:: bash

      myproject serve

.. note::

   For production deployments, see :doc:`deployment`.

Next Steps
----------

* :doc:`configuration` - Learn about configuration options
* :doc:`deployment` - Deploy to production
* :doc:`../reference/cli` - CLI command reference
```

### CLI Reference

```rst
CLI Reference
=============

.. contents:: Commands
   :local:
   :depth: 2

Global Options
--------------

These options are available for all commands:

.. option:: -c, --config <path>

   Path to configuration file. Default: ``~/.config/myproject/config.yaml``

.. option:: -v, --verbose

   Enable verbose output.

.. option:: -q, --quiet

   Suppress non-error output.

.. option:: --version

   Show version information.

Commands
--------

init
^^^^

Initialize a new project.

.. code-block:: bash

   myproject init [options] [name]

**Arguments:**

``name``
   Project name (optional). Defaults to current directory name.

**Options:**

.. option:: --template <name>

   Template to use. Options: ``default``, ``minimal``, ``full``

**Examples:**

.. code-block:: bash

   # Initialize in current directory
   myproject init

   # Initialize with name
   myproject init my-app

   # Initialize with template
   myproject init --template minimal my-app

serve
^^^^^

Start the development server.

.. code-block:: bash

   myproject serve [options]

**Options:**

.. option:: -p, --port <number>

   Port to listen on. Default: ``8080``

.. option:: -h, --host <address>

   Host to bind to. Default: ``localhost``

.. option:: --reload

   Enable hot reload.

**Examples:**

.. code-block:: bash

   # Start on default port
   myproject serve

   # Start on custom port with reload
   myproject serve --port 3000 --reload
```

### API Documentation

```rst
API Reference
=============

.. contents:: Endpoints
   :local:
   :depth: 2

Authentication
--------------

All API requests require authentication using a Bearer token.

.. code-block:: http

   GET /api/v1/resources HTTP/1.1
   Host: api.example.com
   Authorization: Bearer <token>

Resources
---------

List Resources
^^^^^^^^^^^^^^

.. http:get:: /api/v1/resources

   List all resources for the authenticated user.

   **Query Parameters:**

   .. list-table::
      :header-rows: 1

      * - Parameter
        - Type
        - Description
      * - ``limit``
        - integer
        - Maximum number of results (default: 20, max: 100)
      * - ``offset``
        - integer
        - Number of results to skip (default: 0)
      * - ``status``
        - string
        - Filter by status (active, inactive, pending)

   **Example Request:**

   .. code-block:: http

      GET /api/v1/resources?limit=10&status=active HTTP/1.1
      Authorization: Bearer eyJhbGciOiJIUzI1NiIs...

   **Example Response:**

   .. code-block:: json

      {
        "data": [
          {
            "id": "res_123",
            "name": "My Resource",
            "status": "active",
            "created_at": "2024-01-15T10:30:00Z"
          }
        ],
        "pagination": {
          "total": 42,
          "limit": 10,
          "offset": 0
        }
      }

   :statuscode 200: Success
   :statuscode 401: Unauthorized
   :statuscode 500: Internal Server Error

Create Resource
^^^^^^^^^^^^^^^

.. http:post:: /api/v1/resources

   Create a new resource.

   **Request Body:**

   .. code-block:: json

      {
        "name": "My Resource",
        "description": "Optional description"
      }

   **Response:**

   .. code-block:: json

      {
        "id": "res_456",
        "name": "My Resource",
        "description": "Optional description",
        "status": "pending",
        "created_at": "2024-01-15T10:30:00Z"
      }

   :statuscode 201: Created
   :statuscode 400: Invalid request body
   :statuscode 401: Unauthorized
```

### Architecture Decision Record

```rst
ADR-001: Database Selection
===========================

.. admonition:: Status
   :class: note

   **Accepted** - 2024-01-15

Context
-------

We need to select a database for storing application data. The key requirements are:

* ACID compliance for transactional integrity
* JSON support for flexible schemas
* Good performance for read-heavy workloads
* Strong ecosystem and tooling

Decision
--------

We will use **PostgreSQL 16** as our primary database.

Alternatives Considered
-----------------------

.. list-table::
   :header-rows: 1
   :widths: 20 40 40

   * - Option
     - Pros
     - Cons
   * - **PostgreSQL** ✓
     - ACID, JSON support, mature ecosystem
     - Requires schema management
   * - MySQL
     - Familiar, good tooling
     - Weaker JSON support
   * - MongoDB
     - Flexible schema, JSON native
     - Eventual consistency challenges
   * - SQLite
     - Simple, embedded
     - Concurrency limitations

Consequences
------------

**Positive:**

* Strong transactional guarantees
* Excellent JSON/JSONB support
* Rich extension ecosystem (PostGIS, etc.)
* Well-supported by ORMs and migration tools

**Negative:**

* Requires running a separate database server
* Schema migrations needed for changes
* Learning curve for advanced features

**Neutral:**

* Team has existing PostgreSQL experience
```

## Diagram Integration

### PlantUML in RST

```rst
Architecture Overview
=====================

The following diagram shows the system architecture:

.. uml::

   @startuml
   !include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

   Person(user, "User")
   System_Boundary(system, "My System") {
       Container(web, "Web App", "React")
       Container(api, "API", "Go")
       ContainerDb(db, "Database", "PostgreSQL")
   }

   Rel(user, web, "Uses")
   Rel(web, api, "Calls")
   Rel(api, db, "Reads/Writes")
   @enduml
```

### Mermaid in RST

```rst
User Flow
=========

.. mermaid::

   flowchart TD
       A[Start] --> B{Authenticated?}
       B -->|Yes| C[Dashboard]
       B -->|No| D[Login]
       D --> E{Valid?}
       E -->|Yes| C
       E -->|No| F[Error]
       F --> D
```

## Build Commands

```bash
# Build HTML documentation
make html

# Build and serve with auto-reload
make watch

# Check for broken links
make linkcheck

# Build PDF (requires LaTeX)
make latexpdf

# Clean build directory
make clean
```

## Integration with Planning

### Auto-generate from Planning

Create a script to generate docs from planning structure:

```python
#!/usr/bin/env python3
# scripts/generate-docs.py

import os
from pathlib import Path

def generate_requirements_doc(planning_dir: Path, docs_dir: Path):
    """Generate requirements documentation from planning files."""
    req_file = planning_dir / "requirements" / "PRODUCT-REQUIREMENTS.md"
    if req_file.exists():
        # Convert and include in docs
        pass

def generate_architecture_doc(planning_dir: Path, docs_dir: Path):
    """Generate architecture docs from planning files."""
    arch_file = planning_dir / "design" / "ARCHITECTURE.md"
    if arch_file.exists():
        # Convert and include in docs
        pass
```

### Link Planning to Docs

In story files, link to documentation:

```markdown
# Story: S001_user-registration

## Documentation

| Document | Link | Status |
|----------|------|--------|
| API Reference | [docs/reference/api](../../../docs/reference/api.rst) | ✅ |
| User Guide | [docs/guides/registration](../../../docs/guides/registration.rst) | 🟡 |
```

## Hosting Options

| Platform | Pros | Setup |
|----------|------|-------|
| **GitHub Pages** | Free, integrated | GitHub Actions workflow |
| **Read the Docs** | Versioning, search | `.readthedocs.yaml` |
| **Self-hosted** | Full control | Nginx + static files |

### Read the Docs Configuration

```yaml
# .readthedocs.yaml
version: 2

build:
  os: ubuntu-22.04
  tools:
    python: "3.11"

sphinx:
  configuration: docs/conf.py

python:
  install:
    - requirements: docs/requirements.txt
```
