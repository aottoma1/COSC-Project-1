grammar lolcode;

BEGIN_FILE
	:	 '#HAI' | '#hai';
END_FILE:	'#KTHXBYE' | '#kthxbye';
BEGIN_COMMENT
	:	 '#OBTW' | '#obtw';
END_COMMENT
	:	'#TLDR' | '#tldr';
PLAIN	:	'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|'N'|'O'|'P'|'Q'|'R'|'X'|'Y'|'Z'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|','|'.'|'"'|':'|'?'|'%'|'/' |'!';

HEAD_BEGIN
	:	'#MAEK HEAD' | '#maek head';
TITLE_BEGIN
	:	'#GIMMEH TITLE' | '#gimeh title';

P_START	:	'#MAEK PARAGRAF' | '#maek paragraf';
B_START	:	'#GIMMEH BOLD' | '#gimmeh bold';

ITAL_START	:	'#GIMMEH ITALICS' | '#gimmeh italics';
LIST_START
	:	'#MAEK LIST'| '#maek list';
ITEM_START
	:	'#GIMMEH ITEM' | '#gimmeh item';
MKAY_END:	'#MKAY' | '#mkay';
OIC_END	:	'#OIC' | '#oic';
NLINE_START
	:	'#GIMMEH NEWLINE' | '#gimmeh newline';
SOUND_START
	:	'#GIMMEH SOUNDZ' | '#gimmeh soundz';
VID_START
	:	'#GIMMEH VIDZ' | '#gimmeh vidz';
URL	:	('http://www.' | 'HTTP://WWW.');
MP3	:	'.mp3' | 'MP3';
YOUTUBE	:	'http://www.youtube.com' | 'HTTP://WWW.YOUTUBE.COM';
I_HAZ	:	'#I HAZ' | '#i haz';
IT_IZ	:	'#IT IZ' | 'it iz';
SEE	:	'#LEMME SEE' | '#lemme see';



file	:	BEGIN_FILE content? END_FILE;
content	:	 start_head (paragraph | list| bold |italic | newline | comment | sound | video| var_declare| PLAIN | var_access)*;
start_head
	:	comment? head?;
comment	:	 BEGIN_COMMENT comm_text END_COMMENT;
comm_text:	PLAIN+;
head	:	HEAD_BEGIN var_declare? var_access* title var_access* OIC_END comment*;
title	:	TITLE_BEGIN PLAIN+ MKAY_END;
paragraph
	:	P_START var_declare? (bold| italic| list| item| PLAIN | newline | comment | sound |video | var_access)* OIC_END;
bold	:	B_START var_declare? PLAIN* MKAY_END;
italic	:	ITAL_START var_declare? PLAIN* MKAY_END;
list	:	LIST_START item* (var_access |item| comment|newline)* OIC_END;
item	:	ITEM_START var_declare? (PLAIN| bold| italic| var_access)* MKAY_END; 
newline	:	NLINE_START;
sound	:	SOUND_START URL PLAIN* MP3 MKAY_END;
video	:	VID_START YOUTUBE PLAIN* MKAY_END;
var_declare
	:	I_HAZ varname IT_IZ value MKAY_END;
varname	:	PLAIN+ ;
value	:	PLAIN+ ;
var_access
	:	SEE varname MKAY_END;



			










