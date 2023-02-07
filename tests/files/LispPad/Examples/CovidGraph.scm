;;; COVID-19 statistics per country
;;;
;;; This is a suite of tools:
;;;   - to download COVID-19 metrics from the GitHub respository maintained by Johns
;;;     Hopkins University,
;;;   - to convert them into a format stored on local disk,
;;;   - to upload the data into a SQLite database,
;;;   - to query the database for per-country statistics, and
;;;   - to illustrate the results by showing various graphs.
;;;
;;; This example code makes use of a whole range of LispKit libraries: `(lispkit gvector)`,
;;; `(lispkit date-time)`, `(lispkit csv)`, `(lispkit sqlite)`, `(lispkit draw)`, and
;;; `(lisppad system)`.
;;;
;;; Usage:
;;;   1. Create a new empty directory
;;;      (make-directory "~/Desktop/covid")
;;;   2. Download all data from GitHub. This function does not expect an empty directory.
;;;      It will complete the missing data and not touch existing files.
;;;      (download-daily-metrics "~/Desktop/covid")
;;;   3. Read the metric files in the given directory and upload the data into a new SQLite
;;;      database.
;;;      (create-db "~/Desktop/covid.sqlite3" "~/Desktop/covid")
;;;   4. Query the database to obtain a report for the given country.
;;;      (define covid-start (date-time 2020 02 25))
;;;      (define covid-ch (query-country-metrics "~/Desktop/covid.sqlite3" "CH" covid-start))
;;;      (define covid-de (query-country-metrics "~/Desktop/covid.sqlite3" "DE" covid-start))
;;;      (define covid-us (query-country-metrics "~/Desktop/covid.sqlite3" "US" covid-start))
;;;   5. Show country-specific reports.
;;;      (show-country-report covid-ch "CH" 3 12 1000 100)
;;;      (show-country-report covid-de "DE" 3 12 10000 500)
;;;      (show-country-report covid-us "US" 3 12 100000 2500)
;;;
;;; LispPad comes with all the assets needed to run the example code, in case an internet
;;; connection is not available. The directory containing the metric files can be found at
;;; `internal-covid-dir`, the sqlite3 database containing the processed data is located at
;;; `internal-covid-db`. The following adjustments to step 4 show how to use the pre-packaged
;;; assets:
;;;
;;;   4. Query the database to obtain a report for the given country.
;;;      (define covid-start (date-time 2020 02 25))
;;;      (define covid-ch (query-country-metrics internal-covid-db "CH" covid-start))
;;;      (define covid-de (query-country-metrics internal-covid-db "DE" covid-start))
;;;      (define covid-us (query-country-metrics internal-covid-db "US" covid-start))
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

;; Import LispPad system library
(import (lisppad system))

;; Load basic Covid-19 metric management functions (example code from LispKit)
;; http://links.zenger.org/lispkit/Resources/Examples/Covid.scm
(load "Examples/Covid")

;; Opens a new window for displaying graphs showing a report generated via
;; `query-country-metrics` for country `country`. `daystep` is a fixnum determining the
;; interval of showing x-axis labels (dates); e.g. if `daystep` is 3, every third date is
;; shown. `maxlabels`, `cumunit`, and `newunit` are used to specify what labels are drawn
;; and how many.
(define (show-country-report report country daystep maxlabels cumunit newunit)
  (let* ((page (country-report-drawing report country daystep maxlabels cumunit newunit))
         (window (use-graphics-window page
                   (size 800 740)
                   (string-append "COVID-19 statistics for " (region-name country 'en))
                   #f
                   (size 805 785))))
    (set-graphics-window-label! window "Data source: CSSE, Johns Hopkins University")
    window))
