;;; SRFI 14
;;;
;;; The ability to efficiently represent and manipulate sets of characters is an unglamorous
;;; but very useful capability for text-processing code -- one that tends to pop up in the
;;; definitions of other libraries. Hence it is useful to specify a general substrate for this
;;; functionality early. SRFI 14 defines a general library that provides this functionality.
;;;
;;; This implementation is LispKit-specific and is based on characters being UTF16 code units.
;;;
;;; Author of spec: Olin Shivers
;;;
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

(define-library (srfi 14)

  (export char-set?
          char-set=
          char-set<=
          char-set-hash
          char-set-cursor
          char-set-ref
          char-set-cursor-next
          end-of-char-set?
          char-set-fold
          char-set-unfold
          char-set-unfold!
          char-set-for-each
          char-set-map
          char-set-copy
          char-set
          list->char-set
          string->char-set
          list->char-set!
          string->char-set!
          char-set-filter
          ucs-range->char-set
          ->char-set
          char-set-filter!
          ucs-range->char-set!
          char-set->list
          char-set->string
          char-set-size
          char-set-count
          char-set-contains?
          char-set-every
          char-set-any
          char-set-adjoin
          char-set-delete
          char-set-adjoin!
          char-set-delete!
          char-set-complement
          char-set-union
          char-set-intersection
          char-set-complement!
          char-set-union!
          char-set-intersection!
          char-set-difference
          char-set-xor
          char-set-diff+intersection
          char-set-difference!
          char-set-xor!
          char-set-diff+intersection!
          char-set:lower-case
          char-set:upper-case
          char-set:title-case
          char-set:letter
          char-set:digit
          char-set:letter+digit
          char-set:graphic
          char-set:printing
          char-set:whitespace
          char-set:iso-control
          char-set:punctuation
          char-set:symbol
          char-set:hex-digit
          char-set:blank
          char-set:ascii
          char-set:empty
          char-set:full)

  (import (lispkit base)
          (rename (lispkit char-set)
                  (char-set=? char-set=)
                  (char-set<=? char-set<=)
                  (char-set-every? char-set-every)
                  (char-set-any? char-set-any)))
)
