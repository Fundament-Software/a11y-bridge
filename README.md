<!--
SPDX-FileCopyrightText: 2025 2025 Fundament Software SPC <https://fundament.software>

SPDX-License-Identifier: Apache-2.0
-->

Crates
* a11y-models - responsible for providing structs/traits representing a view of data from a11y interfaces such as atspi-bridge
  * eg providing a Document view of a Frame that is displaying a text document that allows retrieving its visible content
* atspi-bridge - responsible for using atspi to extract the state of user interfaces and available actions in a form closely representing atspi
  * TODO: should atspi-bridge be directly responsible for providing in a11y-models format instead rather than indirection?
* a11y-extract - uses atspi-bridge to provide a11y-models data + future alternate protocols
* a11y-capture - responsible for exporting a point in time snapshot of current state
	* Imagine this like a screenshot but over a11y instead of pixels,
	* Useful for recording test data for other crates
	* Stretch goal? would allow recording computer usage locally without high compute cost if called on an interval
		* would need to allowlist what you record for that or something, maybe privacy concerns as local only data can get stolen later
* mcp-server - responsible for exposing an MCP interface to applications with a11y support

Concepts that atspi-bridge should expose

* Applications
	* Frames
	  * TODO
* Actions
