;;; LISPKIT LOG
;;;
;;; Simple logging library for LispKit. This library has been adapted to LispPad to make
;;; use of LispPad's session log. It has the same interface like the original library.
;;; For advanced features, library `(lisppad log)` should be used.
;;;
;;; With library `(lispkit log)`, all log entries are sent to a _logger_. A logger
;;; processes each log entry (e.g. by adding or filtering information) and eventually
;;; persist it if the severity of the log entry is at or above the level of the severity
;;; of the logger. Supported are logging to a port and into a file. A log entry consists
;;; of the following four components: a timestamp, a severity, a sequence of tags, and a
;;; log message. Timestamps are generated via `current-second`. There are five severities
;;; supported by this library: `debug` (0), `info` (1), `warn` (2), `err` (3), and `fatal` (4).
;;; Each tag is represented as a symbol. The sequence of tags is represented as a list of
;;; symbols. A log message is a string.
;;;
;;; Logging functions take the logger as an optional argument. If it is not provided, the
;;; _current logger_ is chosen. The current logger is represented via the parameter object
;;; `current-logger`. The current logger is initially set to the `default-logger`.
;;;
;;; Author: Matthias Zenger
;;; Copyright © 2019 Matthias Zenger. All rights reserved.
;;;
;;; Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
;;; except in compliance with the License. You may obtain a copy of the License at
;;;
;;;   http://www.apache.org/licenses/LICENSE-2.0
;;;
;;; Unless required by applicable law or agreed to in writing, software distributed under the
;;; License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
;;; either express or implied. See the License for the specific language governing permissions
;;; and limitations under the License.

(define-library (lispkit log)

  ;; Severities
  (export severity?
          severity->level
          severity->string
          default-severity)

  ;; Logger datatype
  (export logger?
          logger
          make-logger
          close-logger
          logger-addproc
          logger-severity
          logger-severity-set!)

  ;; Log functions
  (export log
          log-debug
          log-info
          log-warn
          log-error
          log-fatal)

  ;; Logger implementations
  (export default-logger
          current-logger
          make-tag-logger
          make-filter-logger
          make-port-logger
          make-file-logger
          long-log-formatter
          short-log-formatter
          default-log-formatter)

  ;; Syntax extensions
  (export log-using
          log-into-file
          log-from-severity
          log-with-tag
          log-dropping-below-severity
          log-from-severity
          log-time)

  (import (except (lispkit base) log)
          (lisppad log))
)
