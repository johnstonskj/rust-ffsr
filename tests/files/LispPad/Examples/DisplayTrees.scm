;;; Draw binary trees in a graphics window
;;;
;;; This is an extension of the `LispKit` sample code "Examples/DrawTrees.scm". It
;;; loads the `LispKit` sample code and implements a simple function on top which
;;; displays binary trees in `LispPad` graphics windows.
;;; 
;;; Binary trees are represented as s-expressions. An inner node of a binary tree is
;;; represented by a list with tree elements: `(<label> <left tree> <right tree>)`. A
;;; leaf node is just a label. For example: `(1 (2 3 (4 5 #f)) (6 #f 7))` represents
;;; this binary tree:
;;; 
;;;              1
;;;            /   \
;;;           2     6
;;;          / \     \
;;;         3   4     7
;;;            /
;;;           5
;;; 
;;; `test-tree-1`, `test-tree-2`, `test-tree-3`, `test-tree-4`, and ``test-tree-5` are
;;; example trees. They can be displayed in a graphics window via procedure `show-tree`.
;;;
;;; Usage:
;;;   (show-tree test-tree-1)
;;;   (show-tree test-tree-2)
;;;   (show-tree test-tree-3)
;;;   (show-tree test-tree-4)
;;;   (show-tree test-tree-5)
;;; 
;;; Author: Matthias Zenger
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

;; Load the example code
(load "Examples/DrawTrees")

;; `show-tree` displays the given maze `xs` in a graphics window with `title` as title. `fx`
;; and `fy` are scaling factors for x and y coordinates. `pad` is the padding around the tree
;; in pixels.
(define (show-tree xs . args)
  (let-optionals args ((title "Tree layout")
                       (fx 30)
                       (fy 40)
                       (pad 40))
    (let ((node (layout-tree xs 2)))
      (let-values (((xmin xmax ymax) (tree-dimensions node)))
        (let* ((tdrawing (draw-tree node fx fy (- pad (* xmin fx)) pad))
               (dsize    (size (+ (* (- xmax xmin) fx) pad pad)
                               (+ (* ymax fy) pad pad)))
               (wsize    (size (size-width dsize)
                               (+ (size-height dsize) 45)))
               (window   (use-graphics-window tdrawing dsize title #f wsize)))
          (set-graphics-window-label! window "Made with LispPad")
          window)))))
