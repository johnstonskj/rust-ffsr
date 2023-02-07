;;; SCHEME MAPPING
;;;
;;; Library implementing generators. This library is part of the Scheme Tangerine edition of
;;; the R7RS large language.
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

(define-library (scheme mapping)

  (export mapping
      	  mapping/ordered
      	  mapping?
      	  mapping-contains?
      	  mapping-empty?
      	  mapping-disjoint?
      	  mapping-ref
      	  mapping-ref/default
      	  mapping-key-comparator
      	  mapping-adjoin
      	  mapping-adjoin!
      	  mapping-set
      	  mapping-set!
      	  mapping-replace
      	  mapping-replace!
      	  mapping-delete
      	  mapping-delete!
      	  mapping-delete-all
      	  mapping-delete-all!
      	  mapping-intern
      	  mapping-intern!
      	  mapping-update
      	  mapping-update!
      	  mapping-update/default
      	  mapping-update!/default
      	  mapping-pop
      	  mapping-pop!
      	  mapping-search
      	  mapping-search!
      	  mapping-size
      	  mapping-find
      	  mapping-count
      	  mapping-any?
      	  mapping-every?
      	  mapping-keys
      	  mapping-values
      	  mapping-entries
      	  mapping-map
      	  mapping-map->list
      	  mapping-for-each
      	  mapping-fold
      	  mapping-unfold
      	  mapping-unfold/ordered
      	  mapping-filter
      	  mapping-filter!
      	  mapping-remove
      	  mapping-remove!
      	  mapping-partition
      	  mapping-partition!
      	  mapping-copy
      	  mapping->alist
      	  alist->mapping
      	  alist->mapping!
      	  alist->mapping/ordered
      	  alist->mapping/ordered!
      	  mapping=?
      	  mapping<?
      	  mapping>?
      	  mapping<=?
      	  mapping>=?
      	  mapping-union
      	  mapping-intersection
      	  mapping-difference
      	  mapping-xor
      	  mapping-union!
      	  mapping-intersection!
      	  mapping-difference!
      	  mapping-xor!
      	  mapping-min-key
      	  mapping-max-key
      	  mapping-min-value
      	  mapping-max-value
      	  mapping-key-predecessor
      	  mapping-key-successor
      	  mapping-range=
      	  mapping-range<
      	  mapping-range>
      	  mapping-range<=
      	  mapping-range>=
      	  mapping-range=!
      	  mapping-range<!
      	  mapping-range>!
      	  mapping-range<=!
      	  mapping-range>=!
      	  mapping-split
      	  mapping-catenate
      	  mapping-catenate!
      	  mapping-map/monotone
      	  mapping-map/monotone!
      	  mapping-fold/reverse
      	  comparator?
      	  mapping-comparator
      	  make-mapping-comparator)

  (import (srfi 146))
)
