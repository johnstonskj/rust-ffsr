;;; SRFI 23
;;; Error reporting mechanism
;;;
;;; The SRFI introduces a mechanism which allows Scheme code to report errors and abort
;;; the execution.
;;;
;;; Author of spec: Stephan Houben
;;;
;;; Copyright © 2018 Matthias Zenger. All rights reserved.
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

(define-library (srfi 23)

  (export error)

  (import (lispkit dynamic))

  ;; `error` is implemented natively in library `(lispkit dynamic)`
)
