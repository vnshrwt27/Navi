

# **Navi: Code Intelligence & Observability Platform**

### **Project Summary**

Navi is a developer-facing tool designed to **visualize, explore, and analyze large codebases** in an intuitive, interactive way. It converts a codebase into a **graph of nodes and edges** (functions, classes, files, modules as nodes; calls, imports, inheritance as edges) and overlays **runtime behavior, metrics, and dataflow**. The platform enables developers and teams to **understand, query, and optimize software systems**, detect dead code, trace data transformations, and maintain governance.

---

### **Goals**

1. **Visual Code Exploration**

   * Navigate large codebases through interactive graphs.
   * Understand function calls, class hierarchies, and module dependencies.

2. **Queryable Code Graph**

   * Support queries like “which functions are unused?” or “what calls this function?”.
   * Enable both structured (Cypher/GraphQL) and natural language queries.

3. **Dataflow & Observability**

   * Map how data moves across the system.
   * Integrate runtime metrics, logs, and traces to highlight hotspots and errors.

4. **Dead Code & Optimization Insights**

   * Detect unused or rarely executed functions/modules.
   * Track code changes, file churn, and dependency impact.

5. **Access Control & Governance**

   * Restrict visibility of sensitive modules.
   * Track who queries or modifies specific nodes.

6. **Database & Schema Visualization**

   * Map ORM models or SQL/MongoDB schemas to code.
   * Visualize transformations from code → database → downstream functions.

---

### **Core Features**

* AST-based static code analysis (functions, classes, imports, inheritance)
* Runtime observability integration (Prometheus, OpenTelemetry, logs)
* Interactive 2D graph visualization (D3.js, Cytoscape)
* Multi-language support (Tree-sitter / Babelfish)
* Search and query interface (GraphQL, Cypher, optional natural language)
* Metrics, heatmaps, and dashboards for code health
* Dead code detection and code evolution tracking
* Role-based access control for sensitive modules

---

### **Tech Stack**

| Layer                               | Tools / Frameworks                                                          |
| ----------------------------------- | --------------------------------------------------------------------------- |
| **Static Code Analysis**            | Tree-sitter, Python `ast` / `libcst`, Babel (JS/TS), Babelfish (multi-lang) |
| **Graph Storage**                   | Neo4j, ArangoDB, NetworkX (prototyping)                                     |
| **Runtime Metrics / Observability** | Prometheus, OpenTelemetry, Loki/Elasticsearch, Grafana                      |
| **Backend / API**                   | FastAPI, GraphQL, REST                                                      |
| **Frontend / Visualization**        | React, D3.js, Cytoscape.js, Recharts/ECharts                                |
| **Authentication & Access Control** | OAuth2, LDAP, JWT                                                           |
| **Optional AI Layer**               | LLM embeddings for semantic search, natural language queries                |

---

### **Future Extensions**

* LLM-powered explanations & recommendations
* Cross-repo and microservice dependency visualization
* Collaborative exploration and annotation
* CI/CD integration with code evolution monitoring
* Alerting based on runtime anomalies or code changes

Contributors:
Manu
---
