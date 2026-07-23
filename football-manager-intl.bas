1 GOTO 6
5 XZ=PZ:GOTO1690
6 POKE650,127:PIQ$="{blk} {$ab}{$c0}{$c0}{$db}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b3}"
7 IP$="{blk}{$dd}                        {$dd}"
8 RIG$="{rght}{rght}                                    {up}"
9 UNO$="{blk} {$d5}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c9}"
11 DUE$=" {$dd}                                    {$dd}"
13 TRE$=" {$ca}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$cb}{wht}"
15 TUO$=" {$ab}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b3}"
17 REM *****************************
20 REM * FOOTBALL  MANAGER  INTL  *
25 REM *            BY             *
30 REM *     DANIELE PICCOLI       *
36 REM *  LOCALITA' CABINA  N 14   *
37 REM *  GRAZZANO VISCONTI (PC)   *
38 REM *  TEL. 870765 PREF. 0523   *
40 REM *****************************
55 POKE53280,5:POKE53281,5:PRINT"{yel}"
60 PRINTCHR$(142):GOSUB2000:PRINT"{dish}"
90 GOTO570
100 PRINT"{clr}{down}{down}{blk}{$d5}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c9}"
110 FORAPE=1TO16:PRINTIP$:NEXT
120 PRINT"{blk}{$ca}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$cb}{wht}"
125 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}  ^ "
130 IFDES$="G"THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}FOR THE OTHER"
131 IFDES$="G"THENGOTO135
132 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}FOR THE OTHER"
135 IFDES$="G"THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}PLAYERS{wht}"
140 IFDES$="G"THENGOTO150
145 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}TEAMS{wht}"
150 IFDES$="SCE"THENPRINT"{home}           CHOOSE YOUR TEAM"
155 IFDES$="SQ"THENPRINT"{home}WHICH TEAM NAME DO YOU WANT TO CHANGE ?"
160 IFDES$="GIO"THENPRINT"{home}WHICH PLAYER NAME TO CHANGE ?"
170 RETURN
200 GG=0:PRINT"{clr}"
230 PRINTUNO$
231 PRINTDUE$
232 PRINTTRE$
240 GG=GG+1:IFGG<>4THENGOTO230
245 PRINT"{down}{down}"
250 PRINTUNO$
252 PRINTDUE$
254 PRINTDUE$
255 PRINTDUE$
257 PRINTDUE$
258 PRINTDUE$
260 PRINTTRE$ "{down}{down}"
270 RETURN
300 PRINT"{clr}{down}"UNO$:PRINTDUE$:PRINTDUE$
305 PRINTDUE$:PRINTDUE$
310 PRINTTUO$:DUE=0
320 DUE=DUE+1:PRINTDUE$:IFDUE=16THEN340
330 GOTO320
340 PRINTTRE$
345 PRINT"*-INJURED  S-SUBSTITUTE  G-FIRST TEAM":RETURN
350 PRINT"{clr}"UNO$:PRINTDUE$:PRINTTRE$:PRINTUNO$:DUE=0
360 DUE=DUE+1:PRINTDUE$:IFDUE<>7THEN360
365 PRINTTRE$:DUE=0
370 DUE=DUE+1:PRINTUNO$:PRINTDUE$:PRINTDUE$:PRINTDUE$:PRINTTRE$:IFDUE<>2THEN370
375 RETURN
380 PRINT"{clr}"UNO$:PRINTDUE$:PRINTTRE$:PRINTUNO$:DUE=0
390 DUE=DUE+1:PRINTDUE$:IFDUE<>4THEN390
400 PRINTTRE$:RETURN
410 PRINT"{clr}";UNO$:PRINTDUE$:PRINTDUE$:PRINTDUE$:PRINTDUE$:PRINTTRE$:PRINTUNO$
412 DUE=0
415 DUE=DUE+1:PRINTDUE$:PRINTTUO$:IFDUE<>7THEN415
420 PRINTDUE$:PRINTTRE$:RETURN
430 PRINT"{clr}";UNO$:PRINTDUE$:PRINTTUO$:DUE=0
435 DUE=DUE+1:PRINTDUE$:IF DUE<>16THEN435
440 PRINTTUO$:PRINTDUE$:PRINTDUE$:PRINTDUE$:PRINTTRE$
445 RETURN
500 PRINT"{up}{up}{up}{up}{rght}{rght}                                   "
502 PRINT"{rght}{rght}                                   "
503 PRINT"{rght}{rght}                                   "
504 PRINT"{rght}{rght}                                   "
510 RETURN
560 :
570 REM GIOCO PRINCIPALE
580 DIMA$(64),B$(24),C$(30),D$(2),A(24),B(24),C(24),D(14),E(16),F(16),G(16)
590 DIMH(2),J(16),V(16),W(16)
595 I=0:W=15000:Y=0:Z=0:K=15:R=20:B1=1
600 C$="DCA* SG":G(0)=-1
605 FORHZ=1TO16:W(HZ)=0:NEXT
610 FORHZ=1TO24:READB$(HZ):NEXT:PT$="   "
620 DATACOURTOIS,ALISSON,VAN DIJK,HAKIMI,RUDIGER,MARQUINHOS,WALKER,THEO
630 DATADE BRUYNE,MODRIC,BELLINGHAM,PEDRI,KROOS,VALVERDE,ODEGAARD,RODRI
640 DATAMESSI,RONALDO,MBAPPE,HAALAND,SALAH,KANE,VINICIUS,LEWANDOWSKI
650 N=4:FORHZ=1TO64:READA$(HZ):NEXT
660 DATAARSENAL,ATLETICO,BARCELONA,BAYERN,CHELSEA,DORTMUND,INTER,JUVENTUS
670 DATALIVERPOOL,MAN CITY,MAN UNITED,MILAN,NAPOLI,PSG,REAL MADRID,TOTTENHAM
680 DATAAJAX,ATALANTA,BENFICA,FIORENTINA,LAZIO,LEVERKUSEN,LILLE,LYON
690 DATAMARSEILLE,MONACO,NEWCASTLE,PORTO,PSV,ROMA,SEVILLA,VILLARREAL
700 DATAATHLETIC,BETIS,BOLOGNA,CELTIC,FEYENOORD,FRANKFURT,GLADBACH,LEIPZIG
710 DATANICE,RANGERS,RENNES,SOCIEDAD,SPORTING,TORINO,VALENCIA,WOLFSBURG
720 DATAANDERLECHT,ASTON VILLA,BESIKTAS,BRIGHTON,BRUGGE,COPENHAGEN,EVERTON,FENERBAHCE
730 DATAGALATASARAY,LEEDS,OLYMPIACOS,RED STAR,SALZBURG,SHAKHTAR,SPARTA PRAGUE,WEST HAM
735 SR$(4)="D":SR$(3)="C":SR$(2)="B":SR$(1)="A"
740 FORHZ=1TO24:A(HZ)=INT(RND(1)*5)+1
750 B(HZ)=INT(RND(1)*5)+15:NEXT
760 FORHZ=1TO12
770 PZ=INT(RND(1)*24)+1:IFC(PZ)=0THENC(PZ)=4:GOTO790
780 GOTO770
790 NEXT:C(PZ)=3
800 HZ=1
810 IF HZ<>65THENDES$="SCE":GOSUB100
815 IFHZ=65THEN800
820 PRINT"{home}{down}{down}"
830 FORPZ=HZTOHZ+15:PRINTTAB(4)PZ;PT$;A$(PZ):NEXT
840 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}";
850 INPUTA$:IFA$="^"THENHZ=HZ+16:GOTO810
860 QZ=VAL(A$):IFQZ<HZORQZ>HZ+15THEN820
870 A$=A$(49):A$(49)=A$(QZ):A$(QZ)=A$:FORHZ=1TO16:J(HZ)=1:NEXT:J(1)=6:QZ=49
880 REM ******************************
890 PRINT"{clr}{blk}{$d5}{$c0}{$c0}{$b2}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c9}"
900 PRINT"{$dd}{blu}1{blk} {$dd} {wht}TO SELL OR LIST PLAYERS          {blk}{$dd}"PIQ$
910 PRINT"{$dd}{blu}2{blk} {$dd} {wht}TO GET A BANK LOAN               {blk}{$dd}"PIQ$
920 PRINT"{$dd}{blu}3{blk} {$dd} {wht}TO PRINT THE LEAGUE TABLE        {blk}{$dd}"PIQ$
930 PRINT"{$dd}{blu}4{blk} {$dd} {wht}TO SEE THE CLUB STATUS           {blk}{$dd}"PIQ$
953 PRINT"{$dd}{blu}5{blk} {$dd} {wht}TO CHANGE TEAM NAMES             {blk}{$dd}"PIQ$
955 PRINT"{$dd}{blu}6{blk} {$dd} {wht}TO CHANGE PLAYER NAMES           {blk}{$dd}"PIQ$
960 PRINT"{$dd}{blu}G{blk} {$dd} {wht}TO PLAY THE MATCH                {blk}{$dd}"PIQ$
965 PRINT"{$dd}{blu}R{blk} {$dd} {wht}TO RESTART FROM SCRATCH          {blk}{$dd}"
966 PRINT"{$dd}  {$dd} {wht}WITH A DIFFERENT TEAM            {blk}{$dd}"
969 PRINT"{blk}{$ca}{$c0}{$c0}{$b1}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$cb}{wht}"
970 GET A$:IFA$=""THEN970
980 IFA$="1"THENGOSUB1280:GOTO890
990 IFA$="2"THENGOSUB1460:GOTO890
1000 IFA$="3"THENGOSUB1620:GOTO890
1010 IFA$="4"THENGOSUB1760:GOTO890
1020 IFA$="5"THENGOSUB2600:GOTO890
1030 IFA$="6"THENGOSUB2750:GOTO890
1040 IFA$="R"THEN RUN
1060 IFA$<>"G"THEN890
1070 GOSUB2900:REM SCELTA
1080 GOSUB3160:REM PARTITA
1090 GOSUB20590:GOSUB1620:REM RISULTATI & CLASSIFICA
1100 GOSUB3660:REM GUADAGNI
1110 GOSUB3840:REM ACQUISTI
1120 IFI=15THENGOSUB4060:REM FINE STAGIONE
1140 GOTO890
1150 REM LISTA
1160 PZ=0:UZ=0:AZ=0
1170 FORXZ=1TO24:IFC(XZ)>0THENAZ=AZ+1
1180 NEXT
1190 PRINT"{home}{down}{down}{down}{down}{down}"
1210 FORXZ=1TO24:IFC(XZ)=0THEN1260
1211 IFXZ/8<=1THENKJ=1:GOTO1220
1212 IFXZ/8<=2THENKJ=2:GOTO1220
1213 KJ=3
1220 PRINT"{rght}{rght}"CHR$(XZ+64)" "MID$(C$,KJ,1);
1225 PRINT" "B$(XZ)TAB(17);B(XZ)TAB(25);A(XZ)TAB(29);
1230 PRINT500*(5-N)+500*A(XZ);TAB(35)MID$(C$,(C(XZ)+3),1):
1240 IFC(XZ)=3THENPZ=PZ+1:GOTO1260
1250 IFC(XZ)=4THENUZ=UZ+1
1260 NEXT
1270 RETURN
1280 REM VENDITA
1290 GOSUB300:GOSUB1150
1300 PRINT"{home}{down}{rght}{rght}SELL SOMEONE ? IF NOT PRESS SPACE"
1310 GETA$:IFA$=""THEN1310
1315 IFA$=" "THENRETURN:REMPOKE53272,28
1320 IFA$<"A"ORA$>"X"THEN1310
1330 UZ=ASC(A$)-64:PRINT"{home}":REMPOKE53272,28
1340 IFC(UZ)=0THENPRINTRIG$"{rght}{rght}{rght}{rght}"B$(UZ)" IS NOT IN YOUR TEAM{down}{down}{down}":GOTO1430
1350 IFC(UZ)=1THENPRINTRIG$"{rght}{rght}{rght}{rght}THERE ARE NO OFFERS FOR "B$(UZ)"{down}{down}{down}":GOTO1430
1360 PZ=500*(5-N)+500*A(UZ):PZ=INT(PZ+(RND(1)*(PZ/10))-(RND(1)*(PZ/10)))
1365 PRINTRIG$
1370 PRINT"{rght}{rght}ACCEPT "PZ" FROM "A$(INT(RND(1)*64)+1)
1380 PRINT"{rght}{rght}FOR YOUR PLAYER "B$(UZ)" ?"
1390 PRINT"{rght}{rght}PRESS  Y  OR  N  "
1400 GETA$:IFA$<>"N"ANDA$<>"Y"THEN1400
1410 IFA$="Y"THENW=W+PZ:C(UZ)=0:PRINT"{rght}{rght}"B$(UZ)" HAS BEEN SOLD.":GOTO1430
1420 C(UZ)=INT(RND(1)*2)+1:PRINT"{rght}{rght}"B$(UZ)" IS STILL YOUR PLAYER."
1430 PRINT"{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}  SPACE  TO CONTINUE                   "
1440 GETA$:IFA$<>" "THEN1440
1450 RETURN
1460 REM PRESTITO
1465 GOSUB200
1470 PRINT"{home}":PRINT"{rght}{rght}{rght}        BANK OF SPORT"
1475 PRINT"{down}{down}{rght}{rght}{rght}YOU HAVE A TOTAL OF FL. "W
1480 PRINT"{down}{down}{rght}{rght}{rght} AND A DEBT OF FL. "Y
1490 PRINT"{down}{down}{rght}{rght}{rght}HOW MUCH DO YOU WANT TO BORROW{down}{down}"
1500 INPUTXZ:IFXZ=0THENRETURN
1510 IFXZ+Y>10000*(5-N)THENPRINT"{down}{down}{rght}{rght}{rght}IT IS NOT POSSIBLE TO GET"
1515 IFXZ+Y>10000*(5-N)THENPRINT"{rght}{rght}{rght}A LOAN OF FL. "XZ+Y
1520 IFXZ+Y>10000*(5-N)THENPRINT"{rght}{rght}{rght}THE MAX DEBT IN DIVISION "SR$(N)
1525 IFXZ+Y>10000*(5-N)THENPRINT"{rght}{rght}{rght}IS FL. "10000*(5-N)
1530 IFXZ+Y>10000*(5-N)THENGOTO1590
1540 Y=Y+XZ*1.2:Z=Y/20:W=W+XZ
1550 PRINT"{down}{down}{rght}{rght}{rght}YOU HAVE OBTAINED"
1555 PRINT"{rght}{rght}{rght}A LOAN OF FL. "XZ
1560 PRINT"{rght}{rght}{rght}TOTAL DEBT FL. "Y
1570 PRINT"{rght}{rght}{rght}WEEKLY INTEREST FL. "Z
1580 PRINT"{rght}{rght}{rght}YOU HAVE FL. "W
1590 PRINT"{down}{down}  PRESS SPACE "
1600 GETA$:IFA$<>" "THEN1600
1610 RETURN
1620 GOSUB430:REM CLASSIFICA
1630 IFI=0THENGOSUB20810:RETURN:REM NO PARTITE
1640 PRINT"{home}{down}";"{rght}{rght}     TEAM"TAB(21)"F"TAB(25)"A"TAB(29)"PT.";
1650 PRINTTAB(35)"GD{down}"
1660 FORUZ=1TO16:V(UZ)=0:NEXT
1670 FORUZ=1TO16:XZ=0:FORPZ=1TO16
1680 IF(G(PZ)>G(XZ)OR(G(PZ)=G(XZ)AND((F(PZ)-E(PZ))>(F(XZ)-E(XZ)))))ANDV(PZ)=0THEN5
1690 NEXT:V(XZ)=UZ
1693 IFUZ<10THENPRINT"{$a0}";
1695 PRINT"{rght}{rght}";UZ" "A$(XZ+(N-1)*16)TAB(20)F(XZ)TAB(24)E(XZ);
1700 PRINTTAB(28)G(XZ)TAB(34)F(XZ)-E(XZ)
1710 NEXT
1720 PRINT"{down}{rght}{rght}YOUR POSITION IS "V(1)
1723 IF I=1THENPRINT"{rght}{rght}AFTER "I" MATCH ":GOTO1730
1725 PRINT"{rght}{rght}AFTER "I" MATCHES "
1730 PRINT"{rght}{rght}SPACE  TO CONTINUE"
1740 GETA$:IFA$<>" "THEN1740
1750 PRINT"{clr}":RETURN:REMPOKE53272,28
1760 REM CONDIZIONE
1765 GOSUB200
1770 IFB1=1THENPZ=20:GOTO1790
1780 PZ=R/(B1-1)
1785 IFPZ>100THENPZ=100
1790 PRINT"{home}{down}{rght}{rght}{rght}CLUB  "A$(49)
1800 PRINT"{down}{down}{rght}{rght}{rght}MANAGEMENT LEVEL : "PZ
1810 PRINT"{down}{down}{rght}{rght}{rght}SEASONS PLAYED : "B1
1820 PRINT"{down}{down}{rght}{rght}{rght}TEAM MORALE : "K
1830 PRINT"{down}{down}{rght}{rght}{rght}POSITION : "V(1)"'  IN DIVISION "SR$(N)"."
1840 PRINT"{down}{down}{down}{rght}{rght}{rght}MONEY IN BANK : "W" FL."
1850 PRINT"{down}{rght}{rght}{rght}   DEBTS       : "Z
1860 PRINT"{down}{down}{down}{rght}{rght}{rght}PRESS SPACE"
1870 GETA$:IFA$<>" "THEN1870
1880 RETURN
2000 PRINT"{clr}{down}{down}{rvon}                           {rvof}"
2010 PRINT"{rvon} {rvof}{$d5}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c9}{rvon} {rvof}"
2020 PRINT"{rvon} {rvof}{$dd}{wht} FOOTBALL MANAGER INTL {yel}{$dd}{rvon} {rvof}"
2030 PRINT"{rvon} {rvof}{$ab}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b3}{rvon} {rvof}"
2040 PRINT"{rvon} {rvof}{$dd}{wht}   DANIELE  {yel}           {$dd}{rvon} {rvof}"
2050 PRINT"{rvon} {rvof}{$dd}      {wht}       PICCOLI  {yel} {$dd}{rvon} {rvof}"
2070 PRINT"{rvon} {rvof}{$ca}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$cb}{rvon} {rvof}"
2100 PRINT"{rvon}                           {rvof}"
2240 PRINT"{home}{wht}{down}{down}{down}{down}{down}{down}{down}{down}{down}":REM PRESENTAZIONE
2250 PRINT"{$b0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b2}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$ae}"
2260 PRINT"{$dd}           {yel}{$bc}{wht}{$dd}{blu}{$be}{wht}           {$dd}"
2270 PRINT"{$dd}            {$dd}            {$dd}"
2280 PRINT"{$ab}{$c0}{$c0}{$ae} {yel}{$be}{wht}       {$dd}       {blu}{$ac}{wht} {$b0}{$c0}{$c0}{$b3}"
2290 PRINT"{$dd} {yel}{$bb}{wht}{$dd}        {yel}{$bc}{wht}{$dd}{blu}{$be}{wht}        {$dd}{blu}{$ac}{wht} {$dd}"
2300 PRINT"{$ab}{$ae} {$ab}{$c9}       {$ce}{$b7}{$cd}       {$d5}{$b3} {$b0}{$b3}"
2305 PRINT"{yel}{$dd}{wht}{$dd}{$d7}{$dd}{$dd} {yel}{$bb}{wht}    {yel}{$bb}{wht}{$b4}{$d7}{$aa}{blu}{$bc}     {$bb}{wht}{$dd}{$dd}{$d7}{$dd}{blu}{$dd}{wht}"
2310 PRINT"{$ab}{$bd} {$ab}{$cb}       {$cd}{$af}{$ce}       {$ca}{$b3} {$ad}{$b3}"
2320 PRINT"{$dd} {yel}{$be}{wht}{$dd}        {yel}{$ac}{wht}{$dd}{blu}{$bb}{wht}        {$dd}{blu}{$bc}{wht} {$dd}"
2330 PRINT"{$ab}{$c0}{$c0}{$bd}         {$dd}         {$ad}{$c0}{$c0}{$b3}"
2340 PRINT"{$dd}    {yel}{$be}{wht}       {$dd}       {blu}{$bc}{wht}    {$dd}"
2350 PRINT"{$dd}           {yel}{$ac}{wht}{$dd}{blu}{$bb}{wht}           {$dd}"
2360 PRINT"{$ad}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b1}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$bd}{wht}"
2450 FORTR=1TO500:NEXT
2460 RETURN
2599 END
2600 REM CHANGE SQUADRE
2610 HZ=1
2615 IF HZ<>65THEN DES$="SQ":GOSUB100
2620 IFHZ=65THEN2610
2623 PRINT"{home}{down}{down}"
2625 FORPZ=HZTOHZ+15:PRINTTAB(2)PZ;PT$;A$(PZ)
2630 NEXT
2640 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}";
2660 INPUTA$:IFA$="^"THENHZ=HZ+16:GOTO2615
2670 QW=VAL(A$):IFQW<HZORQW>HZ+15THEN2623
2680 PRINT"{clr}{down}{down}{rght}{rght}";A$(QW)
2681 PRINT"  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}"
2682 PRINT"{down}{down}{down}  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}"
2683 PRINT"{down}{down}  TYPE THE NAME"
2687 PRINT"{home}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}":INPUTQW$
2690 IFLEN(QW$)>15THENGOTO2687
2700 A$(QW)=QW$
2710 PRINT"{down}{down}{rght}{rght}";A$(QW)
2712 PRINT"  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}      "
2720 PRINT"{down}{down}{down}{down}PRESS SPACE"
2730 GETA$:IFA$<>" "THEN2730
2740 RETURN
2750 REM CHANGE PLAYER
2760 HZ=1
2765 IF HZ<>25THENDES$="G":GOSUB100
2770 IFHZ=25THEN2760
2780 PRINT"{home}{down}{down}{down}{down}":FORPZ=HZTOHZ+11:PRINTTAB(4)PZ;PT$;B$(PZ):NEXT
2800 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}";
2810 INPUTQW$:IFQW$="^"THENHZ=HZ+12:GOTO2765
2815 QW=VAL(QW$)
2820 IFQW<HZORQW>HZ+11THEN2780
2830 PRINT"{clr}{down}{down}{rght}{rght}";B$(QW)
2832 PRINT"  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}"
2834 PRINT"{down}{down}{down}  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}"
2836 PRINT"{down}{down}  TYPE THE NAME"
2838 PRINT"{home}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}":INPUTWQ$
2840 IFLEN(WQ$)>15THENGOTO2838
2850 B$(QW)=WQ$
2860 PRINT"{down}{down}{rght}{rght}";B$(QW)
2865 PRINT"  {blk}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{$c5}{wht}      "
2870 PRINT"{down}{down}{down}PRESS SPACE"
2880 GETA$:IFA$<>" "THEN2880
2890 RETURN
2900 REM SCELTA
2930 L=1:IFI=0THENWW=INT(RND(1)*2)+1
2935 IFWW=1THENPZ=2:WW=2:GOTO2940
2936 PZ=1:WW=1
2940 SZ=INT(RND(1)*16)+1:IFW(SZ)=1THEN2940
2950 XJ=J(SZ):YJ=PZ:GOSUB20000
2960 IFJ(SZ)/PZ=0THEN2940
2965 IFA$(49)=A$((N-1)*16+SZ)THEN2940
2967 W(SZ)=1
2970 J(SZ)=J(SZ)*PZ:A1=SZ:SZ=(N-1)*16+SZ
2975 IFSZ=49ORSZ>64THEN2900
2980 GOTO3000
3000 IFPZ=2THENA3=1:A4=2:D$(1)=A$(QZ):D$(2)=A$(SZ):GOTO3020
3010 A3=2:A4=1:D$(1)=A$(SZ):D$(2)=A$(QZ)
3020 GOSUB350:PRINT"{home}{down}{rght}{rght} READY TO START THE MATCH ? "
3025 PRINT"{down}{down}{rght}{rght} YOU ARE PLAYING IN THE":PRINT"{down}{rght}{rght} DIVISION "SR$(N)" CHAMPIONSHIP"
3060 PRINT"{down}{rght}{rght}{yel} "D$(1),"{wht}AGAINST{blu}",D$(2)"{wht}"
3100 PRINT"{down}{rght}{rght}    YOU ARE PLAYING ";
3110 IFPZ=2THENPRINT"AT HOME":ZIO$="HOME":GOTO3122
3120 PRINT"AWAY":ZIO$="AWAY"
3122 PRINT"{down}{down}{down}{rght}{rght}{rght}{rght}{rght}         PRESS SPACE"
3125 IFI>0ANDPZ=2THENPRINT"{down}{down}{down}{down}{rght}{rght}{blu}"D$(2);"{wht}  IS "V(A1)" IN THE TABLE"
3127 IFI>0ANDPZ<>2THENPRINT"{down}{down}{down}{down}{rght}{rght}{yel}"D$(1);"{wht}  IS "V(A1)" IN THE TABLE"
3140 GETA$:IFA$<>" "THEN3140
3150 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rvon}SPACE{rvof}":RETURN
3160 REM PARTITA-
3170 GOSUB20050:REMREDO
3180 FORPZ=8TO12
3190 IFL<>1THEND(PZ)=INT(RND(1)*I/3)+15:GOTO3230
3200 IFG(A1)=0ANDI<>0THEND(PZ)=INT(RND(1)*10)+10:GOTO3230
3210 IFG(A1)=0THEND(PZ)=INT(RND(1)*10)+6:GOTO3230
3220 D(PZ)=INT(RND(1)*(G(A1)/I*3)+10)
3230 IFD(PZ)>20THEND(PZ)=20
3240 NEXT
3250 D(13)=INT(D(10)+D(11)/2+D(8)/2+D(9)/2)
3260 D(14)=INT(D(12)+(D(11))/2+(D(8))/2+(D(9))/2)
3270 GOSUB20120:REM NEWVALUE
3275 FORPW=8TO12:IFD(PW)>20THEND(PW)=20
3276 NEXT:FORPW=2TO5:IFD(PW)>20THEND(PW)=20
3277 NEXT
3280 GOSUB350:PRINT"{home}{down}{rght}{rght}"TAB(13)A$(QZ)TAB(25)A$(SZ)
3290 PRINT"{down}{down}{rght}{rght}ENERGY "TAB(15)D(1)TAB(25)D(8)
3300 PRINT"{rght}{rght}MORALE "TAB(15)D(2)TAB(25)D(9)
3310 PRINT"{rght}{rght}DEFENSE"TAB(15)D(3)TAB(25)D(10)
3320 PRINT"{rght}{rght}MIDFIELD"TAB(15)D(4)TAB(25)D(11)
3330 PRINT"{rght}{rght}ATTACK "TAB(15)D(5)TAB(25)D(12)
3340 PRINT"{down}{down}{down}{down}{rght}{rght}YOU HAVE "XZ" PLAYERS IN THE TEAM."
3350 IFUZ=0THENPRINT"{down}{rght}{rght}BUT NO SUBSTITUTE ":GOTO3370
3360 PRINT"{down}{rght}{rght}AND ALSO A SUBSTITUTE "
3370 PRINT"{down}{down}{rght}{rght}   C     TO CHANGE THE LINE-UP"
3380 PRINT"{down}{rght}{rght} SPACE   TO PLAY THE MATCH"
3390 GETA$:IFA$="C"THENGOSUB25070:GOTO3270:REM CAMBIO
3400 IFA$<>" "THEN3390
3410 A$="FIRST HALF":PRINT"{clr}":H(1)=0:H(2)=0
3420 GOSUB20220:REM VISUALIZZA
3430 IFINT(RND(1)*HZ)+1>9THENGOSUB20270:REM CHANCE
3440 IFINT(RND(1)*HZ)+1>3THEN3430
3450 GOSUB20340:REM FINE TEMPO
3460 A$="SECOND HALF":PRINT"{clr}":GOSUB20220
3470 IFINT(RND(1)*HZ)+1>10THENGOSUB20270
3480 IFINT(RND(1)*HZ)+1>3THEN3470
3485 XZ=A1:U1=H(A3):P1=H(A4)
3490 GOSUB20480:REM FINE PARTITA
3510 F(1)=F(1)+U1:E(1)=E(1)+P1:F(A1)=F(A1)+P1:E(A1)=E(A1)+U1:I=I+1
3520 IFU1=P1THENG(1)=G(1)+1:G(A1)=G(A1)+1:GOTO3550
3530 IFU1>P1THENG(1)=G(1)+3:GOTO3550
3540 G(A1)=G(A1)+3
3550 GOTO3650
3630 PRINT"SPACE  TO CONTINUE"
3640 GETA$:IFA$<>" "THEN3640
3650 RETURN
3660 REM GUADAGNI
3670 XZ=0:FORPZ=1TO24:IFC(PZ)>0THENXZ=XZ+7+(5-N)
3680 NEXT
3685 HZ=XZ+50*(5-N)+Z
3690 GOSUB 350:PRINT"{home}{down}{rght}{rght}WEEKLY BALANCE :"
3700 PRINT"{down}{down}{rght}{rght}STADIUM RENT    :"XZ
3710 PRINT"{rght}{rght}SUNDRY EXPENSES :"50*(5-N)
3720 PRINT"{rght}{rght}DEBT INTEREST   :"Z
3730 PRINT"{rght}{rght}TOTAL EXPENSES  :"HZ
3740 PRINT"{rght}{rght}GATE RECEIPTS   :"A2
3750 PRINT"{rght}{rght}WEEKLY BALANCE  :";
3760 PRINTA2-HZ:W=W+A2-HZ
3770 IFW<0THENPRINT"{down}{down}{down}{rght}{rght}THE DEBT IS INCREASED"
3775 IFW<0THENPRINT"{down}{rght}{rght}TO PAY THE STADIUM RENT{up}"
3780 IFW<0THENW=W+100:Y=Y+120:Z=Y/20:GOTO3780
3790 Y=Y-Z:IFY=0THENZ=0
3800 IFW<0THENPRINT"{down}{down}{rght}{rght}YOU HAVE FL. "W:GOTO3805
3802 PRINT"{down}{down}{down}{rght}{rght}YOU HAVE FL. "W
3805 PRINT"{rght}{rght}AND A DEBT OF FL. "Y
3810 PRINT"{rght}{rght}SPACE  TO CONTINUE"
3820 GETA$:IFA$<>" "THEN3820
3830 RETURN
3840 GOSUB200:REM ACQUISTI
3845 REM ACQUISTI
3850 UZ=0:FORPZ=1TO24:IFC(PZ)>0THENUZ=UZ+1
3860 NEXT:IFUZ=16THENGOSUB20860:RETURN
3865 PZ=INT(RND(1)*24)+1:IFC(PZ)<>0THEN3865
3870 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}_   TO CHANGE PLAYER"
3872 PRINT"{rght}{rght}^   TO BUY NOBODY"
3875 PRINT"{home}{down}{rght}{rght}{rght}{rght}{rght}      TRANSFER  MARKET "
3880 PRINT"{down}{down}{rght}{rght}YOU HAVE FL. "W
3900 XZ=500*(5-N)+500*A(PZ)
3910 PRINT"{down}{down}{rght}{rght}"MID$(C$,INT((PZ-1)/8)+1,1)TAB(10)B$(PZ)"           "
3920 PRINT"{down}{down}{rght}{rght}STYLE "A(PZ)TAB(10)"{blk}:{wht}POWER "B(PZ)TAB(22)"{blk}:{wht}VALUE "XZ
3930 PRINT"{down}{down}{down}{down}{down}{rght}{rght}WHAT IS YOUR OFFER"
3940 PRINT"{up}{up}{up}{up}";:INPUTA$:IFA$="^"THENRETURN
3945 IFA$="_"THEN3845
3950 HZ=VAL(A$):IFHZ<=0THEN3870
3960 IFHZ>WTHEN3845
3970 UZ=INT(XZ+RND(1)*(XZ/10)-RND(1)*(XZ/10))
3980 PRINT"{down}{down}"
3990 IFHZ<UZTHENPRINT"{rght}{rght}YOUR OFFER FOR "B$(PZ)
3995 IFHZ<UZTHENPRINT"{rght}{rght}HAS BEEN REJECTED{down}":GOTO4030
4000 PRINT"{rght}{rght}YOUR OFFER FOR "B$(PZ):PRINT"{rght}{rght}HAS BEEN ACCEPTED"
4010 PRINT"{rght}{rght}NOW "B$(PZ)" IS YOUR PLAYER"
4020 C(PZ)=2:W=W-HZ
4030 PRINT"{rght}{rght}PRESS SPACE"
4040 GETA$:IFA$=" "ORA$="_"THEN4045
4042 GOTO 4040
4045 IF A$ ="_"THENGOSUB500:GOTO3845
4050 RETURN
4060 GOSUB430:REM FINE CAMPIONATO
4070 PRINT"{home}{down}{rght}{rght}            END OF SEASON      "
4080 PRINT"{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}SPACE  FOR THE FINAL TABLE"
4090 GETA$:IFA$<>" "THEN4090
4100 GOSUB1620
4110 GOSUB430:PRINT"{home}{down}{down}":R=R+(8-V(1))*5+T*5:B1=B1+1:FORPZ=1TO16
4115 IFV(PZ)>13ANDN<>4THENPRINT"{rght}{rght}RELEGATION FOR ";A$(PZ+(N-1)*16):GOSUB25350
4120 IFV(PZ)<4ANDN<>1THENPRINT"{rght}{rght}PROMOTION FOR ";A$(PZ+(N-1)*16):GOSUB25320
4130 NEXT
4140 IFN=1THENGOSUB25380
4150 PRINT"{rght}{rght}NEW SEASON "
4160 IFV(1)<4ANDN<>1THENN=N-1:GOTO4180
4170 IFV(1)>13ANDN<>4THENN=N+1
4180 PRINT"{rght}{rght}DIVISION "SR$(N):QZ=(N-1)*16+1
4190 FORPZ=1TO24
4200 B(PZ)=INT(RND(1)*5)+15
4210 IFRND(1)>.5THENA(PZ)=A(PZ)+1:GOTO4230
4220 A(PZ)=A(PZ)-1:IFA(PZ)<1THENA(PZ)=1
4230 IFA(PZ)>5THENA(PZ)=5
4240 NEXT
4250 FORPZ=1TO16:V(PZ)=0:J(PZ)=1:E(PZ)=0:F(PZ)=0:G(PZ)=0:W(PZ)=0:NEXT
4260 J(1)=6:K=15:I=0
4270 PRINT"{home}{rght}{rght}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}  G  TO PLAY THE NEW SEASON"
4280 GETA$:IFA$<>"G"THEN4280
4290 RETURN
19999 END
20000 REM SIMULAZIONE ISTRUZIONE DIV
20010 KJ=INT(XJ/YJ):REM
20040 RETURN
20050 REM REDO
20060 FORHZ=1TO24
20070 IFC(HZ)=0THEN20110
20080 IFC(HZ)<3THENB(HZ)=B(HZ)+10
20090 IFC(HZ)=4THENB(HZ)=B(HZ)-1
20093 IFB(HZ)>20THENB(HZ)=20
20095 IFB(HZ)<1THENB(HZ)=1
20097 IFC(HZ)=1ANDB(HZ)>INT(RND(1)*15)THENC(HZ)=2
20100 IFB(HZ)<12ANDRND(1)*B(HZ)<=2THENC(HZ)=1
20110 NEXT:RETURN
20120 REM NEWVALUE
20130 FORPZ=1TO7:D(PZ)=0:NEXT:XZ=0:UZ=0:D(2)=K
20140 FORPZ=1TO24:IFC(PZ)=4THEND(1)=D(1)+B(PZ):XJ=PZ-1:YJ=8:GOSUB20000
20150 IFC(PZ)=4THEND((KJ)+3)=D((KJ)+3)+A(PZ):XZ=XZ+1:GOTO20170
20160 IFC(PZ)=3THENUZ=PZ
20170 NEXT:D(1)=INT(D(1)/11):FORPZ=2TO5
20172 IFD(PZ)>20THEND(PZ)=20
20174 NEXT
20180 D(6)=INT(D(3)+D(4)/2+D(1)/2+D(2)/2)
20190 D(7)=INT(D(5)+(D(6))/2+(D(3))/2+(D(4))/2)
20200 HZ=D(7)-D(13)+D(14)-D(6):IFHZ<10THENHZ=15
20210 RETURN
20220 GOSUB380:REM VISUALIZZA
20230 PRINT"{home}{down}"TAB(8)"MATCH IN PROGRESS :"
20240 PRINT"{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}"D$(1)TAB(20)H(1)
20250 PRINT"{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}"D$(2)TAB(20)H(2)
20260 PRINT"{down}"TAB(26)A$:GOSUB30000:FORTY=1TO1000:NEXT:RETURN
20270 REM CHANCE
20280 IFINT(RND(1)*2)+1=1THEN20310
20290 IFINT(RND(1)*D(14)-RND(1)*D(6))>0THENH(A4)=H(A4)+1:GOSUB46000:GOSUB25000
20300 GOTO20320
20310 IFINT(RND(1)*D(7)-RND(1)*D(13))>0THENH(A3)=H(A3)+1:GOSUB46100:GOSUB25000
20320 RETURN
20340 GOSUB350:REM FINE TEMPO
20350 PRINT"{home}{down}"TAB(7)"THE FIRST HALF IS OVER"
20360 PRINT"{down}{down}{down}{rght}{rght}THE SCORE IS :"
20370 PRINT"{down}{down}{down}{rght}{rght}"D$(1)"  "H(1)"       "D$(2)"  "H(2)
20380 PRINT"{down}{down}{down}{down}{rght}{rght}  C    TO CHANGE THE LINE-UP"
20390 PRINT"{down}{down}{down}{down}{rght}{rght}SPACE  TO PLAY WITHOUT CHANGES"
20400 GETA$:IFA$=" "THENRETURN
20405 IFA$<>"C"THEN20400
20410 GOSUB300:GOSUB1150
20420 PRINT"{home}{down}{rght}{rght}WHO MUST LEAVE THE FIELD ?"
20425 PRINT"{down}{rght}{rght}SPACE TO PLAY WITHOUT SUBSTITUTIONS"
20430 GETA$:IFA$=" "THENPRINT"{clr}":RETURN
20435 IFA$=""THEN20430
20440 PZ=ASC(A$)-64:IF(PZ<0ORPZ>24)THEN20430
20450 IFC(PZ)<>4THEN20430
20460 FORUZ=1TO24:IFC(UZ)=3THENC(UZ)=4
20465 NEXT:C(PZ)=4:GOSUB20120
20470 RETURN
20480 GOSUB410:REM FINE PARTITA
20490 PRINT"{home}{down}"TAB(7)"FINAL SCORE :"
20500 PRINT"{rght}{rght}"D$(1)"  "H(1)"      "D$(2)"  "H(2)
20510 IFU1>P1THENK=INT((20-K)/2)+K:GOTO20530
20515 IFU1=P1THENK=INT(K/2)+1:GOTO20530
20520 K=K-2:IFK<1THENK=1
20530 IFV(1)=0THENV(1)=INT(RND(1)*16)+1
20550 A2=(17-V(1))*INT(RND(1)*40)+50*(5-N)
20560 PRINT"{rght}{rght}GATE RECEIPTS :"A2:K=INT(K)
20570 IFL=1THENC$(I+1)=STR$(SZ)+","+STR$(A3)+"."+STR$(H(A3))+STR$(H(A4))
20580 RETURN
20590 REM RISULTATI
20595 PRINT"{rght}{rght}HERE ARE THE OTHER RESULTS{down}{down}"
20600 FORPZ=1TO16:V(PZ)=0:NEXT
20610 V(1)=1:V(A1)=1:FORPZ=1TO7
20620 XZ=INT(RND(1)*16)+1:IFV(XZ)<>0THEN20620
20630 V(XZ)=1
20640 HZ=INT(RND(1)*16)+1:IFV(HZ)<>0THEN20640
20650 V(HZ)=1
20660 IFG(XZ)<I*2THENSZ=INT(RND(1)*2+I):GOTO20680
20670 SZ=G(XZ)
20680 P1=INT(RND(1)*(SZ/I+3))
20690 IFG(HZ)<I*2THENUZ=INT(RND(1)*2*I):GOTO20710
20700 UZ=G(HZ)
20710 U1=INT(RND(1)*(UZ/I+3)):V(HZ)=1:V(XZ)=1
20720 PRINT"{rght}{rght}";A$((N-1)*16+XZ);TAB(18)P1"  "A$((N-1)*16+HZ)TAB(36)U1
20730 F(HZ)=F(HZ)+U1:F(XZ)=F(XZ)+P1:E(XZ)=E(XZ)+U1:E(HZ)=E(HZ)+P1
20740 IFP1>U1THENG(XZ)=G(XZ)+3:GOTO20770
20750 IFU1>P1THENG(HZ)=G(HZ)+3:GOTO20770
20760 G(XZ)=G(XZ)+1:G(HZ)=G(HZ)+1
20770 PRINT:NEXT
20780 PRINT"{rght}{rght}SPACE  TO  CONTINUE"
20790 GETA$:IFA$<>" "THEN20790
20800 RETURN
20810 PRINT"{home}{down}{down}{down}{rght}{rght}NO MATCHES HAVE BEEN PLAYED"
20820 PRINT"{rght}{rght}IN THIS SEASON YET !"
20830 PRINT"{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}SPACE  TO CONTINUE"
20840 GETA$:IFA$<>" "THEN20840
20850 RETURN
20860 REM NON PIU' GIOC.
20865 PRINT"{home}{down}{rght}{rght}       TRANSFER MARKET           {down}{down}"
20870 PRINT"{rght}{rght}YOU CANNOT HAVE MORE THAN 16        "
20880 PRINT"{down}{down}{rght}{rght}PLAYERS IN YOUR TEAM        "
20885 PRINT"{down}{down}{rght}{rght}UNTIL YOU SELL AT LEAST ONE         "
20887 PRINT"{down}{down}  YOU CANNOT BUY ANY MORE."
20890 PRINT"{down}{down}{rght}{rght}          PRESS SPACE"
20892 PRINT"{down}{down}{down}{down}{down}                                  "
20895 PRINT"                                       "
20900 GETA$:IFA$<>" "THEN20900
20910 RETURN
25000 REM GOAL
25010 FORQI=1TO10
25020 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rvon}{wht}GOAL !{wht}"
25030 FORTY=1TO100:NEXT
25040 PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rvof}{wht}GOAL !{wht}"
25050 FORTY=1TO100:NEXT
25060 NEXT:PRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}"TAB(31)"         ":GOSUB40000:GOSUB20220
25065 RETURN
25070 REM CAMBIO
25080 PRINT"{clr}":GOSUB300:GOSUB1150
25090 PRINT"{home}{down}{rght}{rght}THERE ARE "UZ" PLAYERS IN THE TEAM."
25100 IFPZ=1THENPRINT"{rght}{rght}AND ALSO THE SUBSTITUTE.":GOTO25120
25110 PRINT"{rght}{rght}BUT NOT THE SUBSTITUTE."
25120 IFUZ>11THEN25270
25130 PRINT"{rght}{rght}WHO DO YOU WANT IN THE TEAM ?"
25140 PRINT"{rght}{rght}SPACE TO{$a0}CONTINUE"
25150 GETA$:IFA$=" "THEN25190
25155 IFA$=""THEN25150
25160 HZ=ASC(A$)-64:IFHZ<1ORHZ>24THEN25150
25170 IFC(HZ)<2ORC(HZ)=4THEN25150
25180 C(HZ)=4:GOTO25080
25190 GOSUB300:GOSUB1150
25192 PRINT"{home}{down}{rght}{rght}THERE ARE "UZ" PLAYERS IN THE TEAM."
25194 IFPZ=1THENPRINT"{rght}{rght}AND ALSO THE SUBSTITUTE.":GOTO25200
25196 PRINT"{rght}{rght}BUT NOT THE SUBSTITUTE."
25200 PRINT"{rght}{rght}ENTER THE SUBSTITUTE
25205 PRINT"{rght}{rght}SPACE TO CONTINUE
25210 GETA$:IFA$=" "THENRETURN
25215 IFA$=""THEN25210
25220 HZ=ASC(A$)-64:IFHZ<1ORHZ>24THEN25210
25230 IFC(HZ)<2THEN25210
25240 FORPZ=1TO24:IFC(PZ)=3THENC(PZ)=2
25250 NEXT:C(HZ)=3
25260 GOTO25190
25270 PRINT"{rght}{rght}WHO DO YOU WANT TO REMOVE ?"
25280 GETA$:IFA$=""THEN25280
25285 HZ=ASC(A$)-64:IFHZ<0ORHZ>24THEN25280
25290 IFC(HZ)<>4THEN25280
25300 C(HZ)=2:GOTO25070
25310 RETURN
25320 REM  SQDR PRMSS
25330 A$=A$((N-2)*16+PZ):A$((N-2)*16+PZ)=A$((N-1)*16+PZ):A$((N-1)*16+PZ)=A$:PRINT
25340 RETURN
25350 REM SQDR RTRCSS
25360 A$=A$(N*16+PZ):A$(N*16+PZ)=A$((N-1)*16+PZ):A$((N-1)*16+PZ)=A$:PRINT
25370 RETURN
25380 REM SCUDETTO
25390 FORPZ=1TO16:IFV(PZ)=1THENPRINT"{rght}{rght}";A$(PZ+(N-1)*16)" IS CHAMPION OF EUROPE !
25400 NEXT
25410 RETURN
30000 REM SIMULAZIONE MOVIMENTO
32240 PRINT"{home}{wht}{down}{down}{down}{down}{down}{down}{down}{down}":REM GRAFICA  GIOCO.
32245 PRINT"{rvon}{yel}  "D$(1)"  {wht}"
32250 PRINT"{$b0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b2}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$ae}
32260 PRINT"{$dd}            {$dd}            {$dd}
32270 PRINT"{$dd}            {$dd} {blu}{$bc}{yel}     {$bc}{blu} {$bc}{wht}  {$dd}
32280 PRINT"{$ab}{$c0}{$c0}{$ae}      {blu}{$bc}{wht}  {$dd}         {$b0}{$c0}{$c0}{$b3}
32290 PRINT"{$dd}{yel}{$bc}{wht} {$dd}{blu}{$bc}{yel}    {$bc}{wht}   {$dd}     {yel}{$bc}{wht}   {$dd}{blu}{$bc}{wht} {$dd}
32300 PRINT"{$ab}{$ae} {$ab}{$c9} {yel}{$bc}  {wht}   {$ce}{$b7}{$cd}  {blu} {$bc}{wht}   {$d5}{$b3} {$b0}{$b3}
32305 PRINT"{yel}{$dd}{wht}{$dd}{$d7}{$dd}{$dd}  {blu}{$bc}{wht}    {$b4}{$d7}{$aa}       {$dd}{$dd}{$d7}{$dd}{blu}{$dd}{wht}
32310 PRINT"{$ab}{$bd} {$ab}{$cb}       {$cd}{$af}{$ce}  {yel}{$bc}{wht}    {$ca}{$b3} {$ad}{$b3}
32320 PRINT"{$dd} {yel}{$bc}{wht}{$dd}      {blu} {$bc}{wht} {$dd}{yel}{$bc}{wht}        {$dd}  {$dd}
32330 PRINT"{$ab}{$c0}{$c0}{$bd}         {$dd}        {blu}{$bc}{wht}{$ad}{$c0}{$c0}{$b3}
32340 PRINT"{$dd}   {blu}{$bc}{yel}    {$bc}  {wht} {$dd}  {blu} {$bc}      {yel}{$bc}{wht} {$dd}
32350 PRINT"{$dd}            {$dd}            {$dd}
32360 PRINT"{$ad}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$b1}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$c0}{$bd}
32365 PRINT"{rvon}{blu}  "D$(2)"  {wht}"
32670 RETURN
40000 GIR=0:GIO=INT(RND(1)*3)+1
40010 IFGIO=1ANDNEL=2THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{blu}";:GOTO40500
40020 IFGIO=2ANDNEL=2THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{blu}";:GOTO40500
40030 IFGIO=3ANDNEL=2THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{blu}";:GOTO40500
40040 IFGIO=1ANDNEL=1THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{yel}";:GOTO40500
40050 IFGIO=2ANDNEL=1THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{yel}";:GOTO40500
40060 IFGIO=3ANDNEL=1THENPRINT"{home}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{down}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{rght}{yel}";
40500 PRINT"{left}{$bc}";
41000 FORPIQ=1TO50:NEXT
41500 PRINT"{left}{$ac}";
42000 FORPIQ=1TO50:NEXT
42500 PRINT"{left}{$bb}";
42700 FORPIQ=1TO50:NEXT
43000 PRINT"{left}{$be}";
44000 GIR=GIR+1:IFGIR=20THEN44020
44010 GOTO 40500
44020 PRINT"{wht}":RETURN
46000 IF ZIO$="HOME"THEN NEL=2:RETURN
46010 IF ZIO$="AWAY"THEN NEL=1:RETURN
46020 RETURN
46100 IF ZIO$="HOME"THEN NEL=1:RETURN
46110 IF ZIO$="AWAY"THEN NEL=2:RETURN
46120 RETURN
