# Portfolio refresh implementation plan

**Goal:** Create a polished, fast GitHub Pages portfolio for a Senior AI and Systems Engineer.
**Architecture:** A small Rust static generator renders curated project data into an HTML template; CSS and minimal JavaScript provide responsive presentation and filtering. Generated HTML remains in the publishing root.
**Stack:** Rust standard library, semantic HTML, CSS, vanilla JavaScript, GitHub Pages.

- [ ] Curate accurate public-facing copy from project documentation; avoid private implementation details and unsupported claims.
- [ ] Generate project cards from structured content; preserve direct access to the site without JavaScript.
- [ ] Create an accessible responsive design with portrait, skills, project filters and contact links.
- [ ] Build with Rust, check output and validate browser layouts and interactions.
