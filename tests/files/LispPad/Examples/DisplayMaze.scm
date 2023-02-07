;;; Display mazes in a graphics window
;;;
;;; This is an extension of the `LispKit` sample code "Examples/Maze.scm". It loads the
;;; `LispKit` sample code and implements a simple function on top which displays mazes
;;; in `LispPad` graphics windows.
;;;
;;; Usage:
;;;   (show-maze (make-maze/randomized-dfs 30 30) 15 15 "Maze from randomized DFS algorithm")
;;;   (show-maze (make-maze/bintree 30 30) 15 15 "Maze from binary tree algorithm")
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

;; Load the example code
(load "Examples/Maze")

;; `show-maze` displays the given maze in a graphics window with `title` as title.`dx` and
;; `dy` refer to the size of a single cell. These size numbers are also used to compute the
;; size of the window and the size of the window canvas.
(define (show-maze maze dx dy title)
  (let* ((shape    (maze->shape maze dx dy))
         (mdrawing (drawing (draw (transform-shape shape (translate 20 20)))))
         (dsize    (size (fx+ (fx* (maze-width maze) dx) 50)
                         (fx+ (fx* (maze-height maze) dy) 50)))
         (wsize    (size (size-width dsize) (+ (size-height dsize) 45)))
         (window   (use-graphics-window mdrawing dsize title #f wsize)))
    (set-graphics-window-label! window "Made with LispPad")
    window))
