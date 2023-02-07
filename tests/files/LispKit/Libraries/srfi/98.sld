;;; SRFI 98
;;; An interface to access environment variables
;;;
;;; This SRFI specifies the procedure `get-environment-variable`, which gets the value of the
;;; specified environment variable, and the procedure `get-environment-variables`, which gets
;;; an association list of all environment variables.
;;;
;;; Author of spec: Taro Minowa (Higepon)
;;;
;;; Copyright © 2020 Matthias Zenger. All rights reserved.
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

(define-library (srfi 98)

  (export get-environment-variable
          get-environment-variables)

  (import (lispkit system))

  ;; Both procedures are implemented natively in library `(lispkit system)`
)
