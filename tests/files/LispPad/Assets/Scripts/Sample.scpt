FasdUAS 1.101.10   ��   ��    k             l      ��  ��   ��
 *  Sample AppleScript sub-routines for LispPad
 * 
 *  Author: Matthias Zenger
 *  Copyright � 2020 Matthias Zenger. All rights reserved.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 *  except in compliance with the License. You may obtain a copy of the License at
 *  
 *    http://www.apache.org/licenses/LICENSE-2.0
 *  
 *  Unless required by applicable law or agreed to in writing, software distributed under the
 *  License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
 *  either express or implied. See the License for the specific language governing permissions
 *  and limitations under the License.
      � 	 	| 
   *     S a m p l e   A p p l e S c r i p t   s u b - r o u t i n e s   f o r   L i s p P a d 
   *   
   *     A u t h o r :   M a t t h i a s   Z e n g e r 
   *     C o p y r i g h t   �   2 0 2 0   M a t t h i a s   Z e n g e r .   A l l   r i g h t s   r e s e r v e d . 
   * 
   *     L i c e n s e d   u n d e r   t h e   A p a c h e   L i c e n s e ,   V e r s i o n   2 . 0   ( t h e   " L i c e n s e " ) ;   y o u   m a y   n o t   u s e   t h i s   f i l e 
   *     e x c e p t   i n   c o m p l i a n c e   w i t h   t h e   L i c e n s e .   Y o u   m a y   o b t a i n   a   c o p y   o f   t h e   L i c e n s e   a t 
   *     
   *         h t t p : / / w w w . a p a c h e . o r g / l i c e n s e s / L I C E N S E - 2 . 0 
   *     
   *     U n l e s s   r e q u i r e d   b y   a p p l i c a b l e   l a w   o r   a g r e e d   t o   i n   w r i t i n g ,   s o f t w a r e   d i s t r i b u t e d   u n d e r   t h e 
   *     L i c e n s e   i s   d i s t r i b u t e d   o n   a n   " A S   I S "   B A S I S ,   W I T H O U T   W A R R A N T I E S   O R   C O N D I T I O N S   O F   A N Y   K I N D , 
   *     e i t h e r   e x p r e s s   o r   i m p l i e d .   S e e   t h e   L i c e n s e   f o r   t h e   s p e c i f i c   l a n g u a g e   g o v e r n i n g   p e r m i s s i o n s 
   *     a n d   l i m i t a t i o n s   u n d e r   t h e   L i c e n s e . 
     
  
 l     ��������  ��  ��        l     ��  ��      Safari sub-routines     �   (   S a f a r i   s u b - r o u t i n e s      l     ��������  ��  ��        i         I      ��������  0 safarifronturl safariFrontURL��  ��    O        L       n        1    
��
�� 
pURL  4   �� 
�� 
docu  m    ����   m       x                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��        l     ��������  ��  ��       !   i     " # " I      �� $���� &0 setsafarifronturl setSafariFrontURL $  %�� % o      ���� 0 newurl newUrl��  ��   # O     & ' & r     ( ) ( o    ���� 0 newurl newUrl ) n       * + * 1   	 ��
�� 
pURL + 4   	�� ,
�� 
docu , m    ����  ' m      - -x                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��   !  . / . l     ��������  ��  ��   /  0 1 0 i     2 3 2 I      �������� 0 
safariurls 
safariURLs��  ��   3 O     P 4 5 4 k    O 6 6  7 8 7 r    	 9 : 9 2    ��
�� 
cwin : o      ���� 
0 allwin   8  ; < ; r   
  = > = J   
 ����   > o      ���� 0 urls   <  ? @ ? X    M A�� B A k    H C C  D E D r    # F G F J    !����   G o      ���� 0 winurls   E  H I H X   $ C J�� K J r   6 > L M L l  6 ; N���� N c   6 ; O P O n   6 9 Q R Q 1   7 9��
�� 
pURL R o   6 7���� 0 t   P m   9 :��
�� 
TEXT��  ��   M n       S T S  ;   < = T o   ; <���� 0 winurls  �� 0 t   K n   ' * U V U 2   ( *��
�� 
bTab V o   ' (���� 0 win   I  W�� W r   D H X Y X o   D E���� 0 winurls   Y n       Z [ Z  ;   F G [ o   E F���� 0 urls  ��  �� 0 win   B o    ���� 
0 allwin   @  \�� \ o   N O���� 0 urls  ��   5 m      ] ]x                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��   1  ^ _ ^ l     ��������  ��  ��   _  ` a ` i     b c b I      �������� "0 safarifronthtml safariFrontHTML��  ��   c O     d e d I   �� f g
�� .sfridojsnull���     ctxt f m     h h � i i D d o c u m e n t . d o c u m e n t E l e m e n t . i n n e r H T M L g �� j��
�� 
dcnm j 4   
�� k
�� 
docu k m    	���� ��   e m      l lx                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��   a  m n m l     ��������  ��  ��   n  o p o i     q r q I      �������� 20 safarifrontwindowbounds safariFrontWindowBounds��  ��   r O     s t s e     u u l    v���� v n     w x w 1    
��
�� 
pbnd x l    y���� y 4   �� z
�� 
cwin z m    ���� ��  ��  ��  ��   t m      { {x                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��   p  | } | l     ��������  ��  ��   }  ~  ~ i     � � � I      �� ����� 80 setsafarifrontwindowbounds setSafariFrontWindowBounds �  ��� � o      ���� 0 bnds  ��  ��   � O     � � � r     � � � o    ���� 0 bnds   � l      ����� � n       � � � 1   	 ��
�� 
pbnd � l   	 ����� � 4   	�� �
�� 
cwin � m    ���� ��  ��  ��  ��   � m      � �x                                                                                  sfri  alis      Internal                       BD ����
Safari.app                                                     ����            ����  
 cu             Applications  /:Applications:Safari.app/   
 S a f a r i . a p p    I n t e r n a l  Applications/Safari.app   / ��     � � � l     ��������  ��  ��   �  � � � l     �� � ���   �   Music sub-routines    � � � � &   M u s i c   s u b - r o u t i n e s �  � � � l     ��������  ��  ��   �  � � � i     � � � I      �� ����� (0 searchmusiclibrary searchMusicLibrary �  ��� � o      ���� 0 txt  ��  ��   � O     B � � � k    A � �  � � � r     � � � I   �� � �
�� .hookSrchnull���     cPly � 4    �� �
�� 
cLiP � m    ����  � �� ���
�� 
pTrm � o   	 
���� 0 txt  ��   � o      ���� 	0 trcks   �  � � � r     � � � J    ����   � o      ���� 0 res   �  � � � X    ? ��� � � r   & : � � � J   & 7 � �  � � � n   & ) � � � 1   ' )��
�� 
pPIS � o   & '���� 0 trck   �  � � � n   ) , � � � 1   * ,��
�� 
pidx � o   ) *���� 0 trck   �  � � � n   , / � � � 1   - /��
�� 
pnam � o   , -���� 0 trck   �  � � � n   / 2 � � � 1   0 2��
�� 
pArt � o   / 0���� 0 trck   �  ��� � n   2 5 � � � 1   3 5��
�� 
pAlb � o   2 3���� 0 trck  ��   � n       � � �  ;   8 9 � o   7 8���� 0 res  �� 0 trck   � o    ���� 	0 trcks   �  ��� � o   @ A�� 0 res  ��   � m      � ��                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��   �  � � � l     �~�}�|�~  �}  �|   �  � � � i     � � � I      �{�z�y�{ $0 countmusictracks countMusicTracks�z  �y   � O     � � � I   �x ��w
�x .corecnte****       **** � l   
 ��v�u � n    
 � � � 2    
�t
�t 
cTrk � 4    �s �
�s 
cLiP � m    �r�r �v  �u  �w   � m      � ��                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��   �  � � � l     �q�p�o�q  �p  �o   �  � � � i     # � � � I      �n ��m�n  0 getmusictracks getMusicTracks �  � � � o      �l�l 
0 istart   �  ��k � o      �j�j 0 iend  �k  �m   � O     A � � � k    @ � �  � � � r     � � � J    �i�i   � o      �h�h 0 res   �  � � � r   	  � � � o   	 
�g�g 
0 istart   � o      �f�f 0 i   �  � � � V    > � � � k    9 � �  � � � r     � � � n     � � � 4    �e �
�e 
cTrk � o    �d�d 0 i   � 4    �c �
�c 
cLiP � m    �b�b  � o      �a�a 0 trck   �  � � � r    3 � � � J    0 � �  �  � n    " 1     "�`
�` 
pPIS o     �_�_ 0 trck     n   " % 1   # %�^
�^ 
pidx o   " #�]�] 0 trck    n   % (	
	 1   & (�\
�\ 
pnam
 o   % &�[�[ 0 trck    n   ( + 1   ) +�Z
�Z 
pArt o   ( )�Y�Y 0 trck   �X n   + . 1   , .�W
�W 
pAlb o   + ,�V�V 0 trck  �X   � n        ;   1 2 o   0 1�U�U 0 res   � �T r   4 9 [   4 7 o   4 5�S�S 0 i   m   5 6�R�R  o      �Q�Q 0 i  �T   � A     o    �P�P 0 i   o    �O�O 0 iend   � �N o   ? @�M�M 0 res  �N   � m     �                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��   �  l     �L�K�J�L  �K  �J     i   $ '!"! I      �I�H�G�I  0 allmusictracks allMusicTracks�H  �G  " O     >#$# k    =%% &'& r    ()( n    
*+* 2    
�F
�F 
cTrk+ 4    �E,
�E 
cLiP, m    �D�D ) o      �C�C 	0 trcks  ' -.- r    /0/ J    �B�B  0 o      �A�A 0 res  . 121 X    ;3�@43 r   " 6565 J   " 377 898 n   " %:;: 1   # %�?
�? 
pPIS; o   " #�>�> 0 trck  9 <=< n   % (>?> 1   & (�=
�= 
pidx? o   % &�<�< 0 trck  = @A@ n   ( +BCB 1   ) +�;
�; 
pnamC o   ( )�:�: 0 trck  A DED n   + .FGF 1   , .�9
�9 
pArtG o   + ,�8�8 0 trck  E H�7H n   . 1IJI 1   / 1�6
�6 
pAlbJ o   . /�5�5 0 trck  �7  6 n      KLK  ;   4 5L o   3 4�4�4 0 res  �@ 0 trck  4 o    �3�3 	0 trcks  2 M�2M o   < =�1�1 0 res  �2  $ m     NN�                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��    OPO l     �0�/�.�0  �/  �.  P QRQ i   ( +STS I      �-U�,�-  0 playmusictrack playMusicTrackU V�+V o      �*�* 	0 title  �+  �,  T O    WXW I   �)Y�(
�) .hookPlaynull��� ��� obj Y n    Z[Z 4    �'\
�' 
cTrk\ o   	 
�&�& 	0 title  [ 4    �%]
�% 
cLiP] m    �$�$ �(  X m     ^^�                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��  R _`_ l     �#�"�!�#  �"  �!  ` aba i   , /cdc I      � e��  $0 playmusictrackid playMusicTrackIde f�f o      �� 0 idnt  �  �  d O     &ghg k    %ii jkj r    lml 6   non n    pqp 4   �r
� 
cTrkr m   	 
�� q 4    �s
� 
cLiPs m    �� o =   tut 1    �
� 
pPISu o    �� 0 idnt  m o      �� 0 t  k v�v I   %�w�
� .hookPlaynull��� ��� obj w n    !xyx 4    !�z
� 
cTrkz l    {��{ n     |}| 1    �
� 
pidx} o    �� 0 t  �  �  y 4    �~
� 
cLiP~ m    �� �  �  h m     �                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��  b ��� l     ��
�	�  �
  �	  � ��� i   0 3��� I      ���� 0 	stopmusic 	stopMusic�  �  � O    
��� I   	���
� .hookStopnull��� ��� null�  �  � m     ���                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��  � ��� l     ��� �  �  �   � ��� i   4 7��� I      �������� 0 togglemusic toggleMusic��  ��  � O    
��� I   	������
�� .hookPlPsnull��� ��� null��  ��  � m     ���                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��  � ��� l     ��������  ��  ��  � ��� i   8 ;��� I      ������� "0 playmusicstream playMusicStream� ���� o      ���� 0 link  ��  ��  � O    
��� I   	�����
�� .GURLGURLnull��� ��� ctxt� o    ���� 0 link  ��  � m     ���                                                                                      @ alis    $  Internal                       BD ����	Music.app                                                      ����            ����  
 cu             Applications   /:System:Applications:Music.app/   	 M u s i c . a p p    I n t e r n a l  System/Applications/Music.app   / ��  � ��� l     ��������  ��  ��  � ��� i   < ?��� I     ������
�� .aevtoappnull  �   � ****��  ��  � I    �����
�� .sysodisAaleR        TEXT� m     �� ��� � T h i s   s a m p l e   s c r i p t   s h o u l d   n o t   b e   r u n .   I t   p r o v i d e s   A p p l e S c r i p t   s u b - r o u t i n e s   t o   L i s p P a d .��  � ���� l     ��������  ��  ��  ��       ���������������������  � ����������������������������������  0 safarifronturl safariFrontURL�� &0 setsafarifronturl setSafariFrontURL�� 0 
safariurls 
safariURLs�� "0 safarifronthtml safariFrontHTML�� 20 safarifrontwindowbounds safariFrontWindowBounds�� 80 setsafarifrontwindowbounds setSafariFrontWindowBounds�� (0 searchmusiclibrary searchMusicLibrary�� $0 countmusictracks countMusicTracks��  0 getmusictracks getMusicTracks��  0 allmusictracks allMusicTracks��  0 playmusictrack playMusicTrack�� $0 playmusictrackid playMusicTrackId�� 0 	stopmusic 	stopMusic�� 0 togglemusic toggleMusic�� "0 playmusicstream playMusicStream
�� .aevtoappnull  �   � ****� �� ����������  0 safarifronturl safariFrontURL��  ��  �  �  ����
�� 
docu
�� 
pURL�� � 
*�k/�,EU� �� #���������� &0 setsafarifronturl setSafariFrontURL�� ����� �  ���� 0 newurl newUrl��  � ���� 0 newurl newUrl�  -����
�� 
docu
�� 
pURL�� � 
�*�k/�,FU� �� 3���������� 0 
safariurls 
safariURLs��  ��  � ������������ 
0 allwin  �� 0 urls  �� 0 win  �� 0 winurls  �� 0 t  �  ]��������������
�� 
cwin
�� 
kocl
�� 
cobj
�� .corecnte****       ****
�� 
bTab
�� 
pURL
�� 
TEXT�� Q� M*�-E�OjvE�O =�[��l kh jvE�O ��-[��l kh ��,�&�6F[OY��O��6F[OY��O�U� �� c���������� "0 safarifronthtml safariFrontHTML��  ��  �  �  l h������
�� 
dcnm
�� 
docu
�� .sfridojsnull���     ctxt�� � ��*�k/l U� �� r���������� 20 safarifrontwindowbounds safariFrontWindowBounds��  ��  �  �  {����
�� 
cwin
�� 
pbnd�� � 	*�k/�,EU� �� ����������� 80 setsafarifrontwindowbounds setSafariFrontWindowBounds�� ����� �  ���� 0 bnds  ��  � ���� 0 bnds  �  �����
�� 
cwin
�� 
pbnd�� � 
�*�k/�,FU� �� ����������� (0 searchmusiclibrary searchMusicLibrary�� ����� �  ���� 0 txt  ��  � ���������� 0 txt  �� 	0 trcks  �� 0 res  �� 0 trck  �  �������������������������
�� 
cLiP
�� 
pTrm
�� .hookSrchnull���     cPly
�� 
kocl
�� 
cobj
�� .corecnte****       ****
�� 
pPIS
�� 
pidx
�� 
pnam
�� 
pArt
�� 
pAlb�� �� C� ?*�k/�l E�OjvE�O (�[��l kh ��,��,��,��,��,�v�6F[OY��O�U� �� ����������� $0 countmusictracks countMusicTracks��  ��  �  �  �������
�� 
cLiP
�� 
cTrk
�� .corecnte****       ****�� � *�k/�-j U� �� ���~���}��  0 getmusictracks getMusicTracks� �|��| �  �{�z�{ 
0 istart  �z 0 iend  �~  � �y�x�w�v�u�y 
0 istart  �x 0 iend  �w 0 res  �v 0 i  �u 0 trck  � 	�t�s�r�q�p�o�n�m
�t 
cLiP
�s 
cTrk
�r 
pPIS
�q 
pidx
�p 
pnam
�o 
pArt
�n 
pAlb�m �} B� >jvE�O�E�O 0h��*�k/�/E�O��,��,��,��,��,�v�6FO�kE�[OY��O�U� �l"�k�j���i�l  0 allmusictracks allMusicTracks�k  �j  � �h�g�f�h 	0 trcks  �g 0 res  �f 0 trck  � N�e�d�c�b�a�`�_�^�]�\�[
�e 
cLiP
�d 
cTrk
�c 
kocl
�b 
cobj
�a .corecnte****       ****
�` 
pPIS
�_ 
pidx
�^ 
pnam
�] 
pArt
�\ 
pAlb�[ �i ?� ;*�k/�-E�OjvE�O (�[��l kh ��,��,��,��,��,�v�6F[OY��O�U� �ZT�Y�X���W�Z  0 playmusictrack playMusicTrack�Y �V��V �  �U�U 	0 title  �X  � �T�T 	0 title  � ^�S�R�Q
�S 
cLiP
�R 
cTrk
�Q .hookPlaynull��� ��� obj �W � *�k/�/j U� �Pd�O�N���M�P $0 playmusictrackid playMusicTrackId�O �L��L �  �K�K 0 idnt  �N  � �J�I�J 0 idnt  �I 0 t  � �H�G��F�E�D
�H 
cLiP
�G 
cTrk�  
�F 
pPIS
�E 
pidx
�D .hookPlaynull��� ��� obj �M '� #*�k/�k/�[�,\Z�81E�O*�k/��,E/j U� �C��B�A���@�C 0 	stopmusic 	stopMusic�B  �A  �  � ��?
�? .hookStopnull��� ��� null�@ � *j U� �>��=�<���;�> 0 togglemusic toggleMusic�=  �<  �  � ��:
�: .hookPlPsnull��� ��� null�; � *j U� �9��8�7���6�9 "0 playmusicstream playMusicStream�8 �5��5 �  �4�4 0 link  �7  � �3�3 0 link  � ��2
�2 .GURLGURLnull��� ��� ctxt�6 � �j U� �1��0�/���.
�1 .aevtoappnull  �   � ****�0  �/  �  � ��-
�- .sysodisAaleR        TEXT�. �j ascr  ��ޭ