SQLite format 3   @    g                                                              g .@   í W9í                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           IitablekunyomikunyomiCREATE TABLE kunyomi (
                  id              INTEGER PRIMARY KEY,
                  kunyomi         TEXT NOT NULL,
                  okurigana_index INTEGER,
                  kanji_id        INTEGER NOT NULL,
                  FOREIGN KEY(kanji_id) REFERENCES kanji(id)
                  )tableonyomionyomiCREATE TABLE onyomi (
                  id              INTEGER PRIMARY KEY,
                  onyomi          TEXT NOT NULL,
                  kanji_id        INTEGER NOT NULL,
                  FOREIGN KEY(kanji_id) REFERENCES kanji(id)
                  )&+tablekanjikanjiCREATE TABLE kanji (
                  id              INTEGER PRIMARY KEY,
                  kanji           TEXT NOT NULL
                  )   Qx øðèàØÐÈÀ¸°¨ xph`XPH@80(  øðèàØÐÈÀ¸°¨ xph`XPH@80(  øðèàØÐÈÀ¸°¨ x                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Q å­P æO åN ç«M å¤L ç®K æ¨J åI æ¬H æG ç¾F å«E ç½D å¹´C å¥B æ¥A äº@ å? ç°> å¤©= çº< è«; ä¸­: ç«¹9 ç·8 å¤§7 æ6 è¶³5 è4 æ©3 å·2 å1 å0 èµ¤/ ç³. é- ç, æ­£+ æ°´* äºº) æ£®( ä¸' å°& å¥³% åº$ å# æ" è»! ä¸  è³ å­ ç³¸ å­ å å±± ä¸ å·¦ æ ¡ å£ äº è¦ ç¬ æ ç©º ä¹ é ç ä¼ ä¹ æ° å­¦
 è²	 è± ç« ä¸ é³ ç å é¨ å³ ä¸   t
 ôèÞÑÇº­ uk^TJ:0 öæÜÏÂµ¨zmcVI?5+! óæÙÉ¹¬rbRE8+ñäÔÇºªsfYL?2%þñä×Ç·§}si_RE5(
þ
î
á
Ô
Ç
·
ª



v
i
\
L
?
/
"
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     t ã­ã¯Qs ãªã³Pr ãªã§ã¯Oq ãªã­Op ãªã¥ã¦No ãªãNn ã»ã­Mm ã¢ã¯Ll ãã¯Lk ã¢ã¯Kj ãã¯Ki ã¡ã¤Jh ãã§ã¦Jg ãã³If ã¢ã³He ãã³Hd ãã£ã¯Gc ããFb ãã£ã¯Ea ãã¯E` ãã³D_ ãã¥ã¦C^ ããB] ã¸ãB\ ãA[ ã@Z ã@Y ãã³?X ãã³>W ãã§ã¦=V ãã¥ã¦<U ãã¥ã¦;T ã¸ã¥ã¦;S ãã¯:R ãã³9Q ãã³9P ãã¤8O ã¿ã¤8N ã½ã³7M ã½ã¯6L ã½ã¦5K ã½ã¦4J ãµã4I ã»ã³3H ã»ã³2G ã»ã³1F ã»ã­0E ã·ã£ã¯0D ã»ã­/C ã·ã£ã¯/B ã³ã¯/A ã»ã¤.@ ã·ã§ã¦.? ã»ã¤-> ã·ã§ã¦-= ã»ã¤,< ã·ã§ã¦,; ã¹ã¤+: ãã³*9 ã¸ã³*8 ã·ã³)7 ã¸ã§ã¦(6 ã·ã§ã¦(5 ã·ã§ã¦'4 ãã§ã¦&3 ãã§&2 ã¸ã§&1 ã¹ã¤%0 ã·ã¥ã%/ ã¸ã¥ã¦$. ã¸ã$- ã·ã¥#, ã·ã£"+ ã·ã!* ã¸ ) ã¸( ã·' ã¹& ã·% ã·$ ãµã³# ãµã³" ãµ! ã³ã¦  ã³ã¦ ã¯ ã´ ã±ã³ ã±ã³ ã¬ã ã²ã ã¯ã¦ ã¯ ã­ã¥ã¦ ã³ã³ ã­ã³ ã®ã§ã¯ ã­ã¥ã¦ ã¯ ã­ã¥ã¦ ã± ã­ ã¬ã¯ ã«	 ã« ã²
 ã«	 ã¤ã³ ãªã³ ãªã¦ ã¨ã³ ã¦ ã¦ã¦ ã¦
 	ã¤ã
 	ã¤ã   q óâÔÆ¸ªn]H6$ôæØÍÂ´¦jXC5'óåÔÆ¸ªzl^L>-"ôéÛÉ»°¢xfXG<1&
üëàÕÊµ¤wiWB0"
ñ
ß
Ñ
À
¯


|
n
c
U
D
3
%

	÷	å	Ð	¸	ª				j	U	G	9	+		þðÛÆµ§}oaVG8%þìÙÊ»¬~k\M>2&óäÒÀ®¢q                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            ãã£ã¤Q 	ãã¤Q  ããQ	  ãQ  ã¯ããP  ã¡ããO 	ãã¦ãN 	ãã¤N  ããM	  ãL	  ã¾L	  ãK	  ãK	  ãªJ  ãã¨I  ãµã¿H  ããF ãã£ã¤F 	ãã¤F	  ãF ãããE
  ããE	  ããE  ã¨ãD ã¯ããC 	ãããC 	ããC	  ã²B	  ãB ãµãã¤A  ãµãA   ã¤ã¡@	  ã?~  ãã>}  ãã¾>|  ã¾ã¡={  ãã<z  ãªã;y  ãã:x  ãã¨ã9w %ãããã8v %ãããã«8u  ãã8t  ãã7s 	ãã6r 	ããã6q 	ãã6p  ãã6o  ãã5n %ã¯ããã4m %ã¯ãã¾ã4l ã¯ãã4k  ãã3	j  ã¡2i  ãã1h +ããããã0g %ãããã0f ããã0e  ãã0d  ãã/c ããã.b  ãã.a 	ã¯ãã-` 	ã¯ãã-_  ãªã¾-	^  ã-] 	ãã-\ 	ãã-[ %	ãã¾ãã-Z 	ããã-Y 	ããã-X 	ããã-W  ã¾ã,V ãã ã,U %ãã ãã,T  ã¿ã+S  ã²ã¨*R  ãã)Q ã®ã¼ã(P %ã®ã¼ãã(O ã®ã¼ã(N  ãã¿(M  ãã(L  ãã(K 	ããã(J 	ããã(I %ã¡ããã'	H  ã'	G  ã'	F  ã&E  ãããª&D 	ã§ã%C 	ã ã%B  ã¨ã$	A  ã¨$	@  ã¦#	?  ã#>  ããã¾"=  ãªã®!< ãªãªã¤!;  ãªãª!:  ã¿ã¿ 9  ãã8  ãã¨	7  ã6  ãã5 ãã£ã¤4 	ãã¤	3  ã2  ãã¾1 ã¿ã£ã¤0 	ã¿ã¤	/  ã¿.  ã²ã ã-  ãã¡, ãã¤ã¤+  ãã¤* 	ã¿ã) 	ã¿ãã( 	ã¿ãã'  ãã¬&  ã¤ã%  ãã$  ãã# 	ããã" 	ãã! %ããã®ã¤   ããã®  ãã­  ããª  ãã¾ %ãããã ããã %ããã¾ã %ããã®ã¤ ã¾ãªã¶  ãã
  ã¯ãª		  ã»	  ã²  ãã¨  ãã  ãã 	ããã 	ããã ãã ã ãã ã %ãã ãã 	ããã
 	ããã		  ã­  ãã¨ ã¾ãã  ã¾ã  ãã  ãã¾  ã¿ã 	ã²ã¨ã¤  	ã²ã¨