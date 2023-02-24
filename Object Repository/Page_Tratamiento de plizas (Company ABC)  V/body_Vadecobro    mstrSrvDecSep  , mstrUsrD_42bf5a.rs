<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a</name>
   <tag></tag>
   <elementGuidId>b77de70d-703e-4ac2-a7c1-93e66ad5a777</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>10fd88db-f0ad-47d9-ad62-9bf60e733d06</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onunload</name>
      <type>Main</type>
      <value>closeWindows();</value>
      <webElementGuid>40b51fc9-7e5f-470a-be0b-d7425fe45487</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Vía de cobro    mstrSrvDecSep = ','; mstrUsrDecSep = ','; top.frames[&quot;fraSequence&quot;].pintZone=2;
var lblnQuery;
var lintpos;
lintpos = top.document.title.search(&quot; / &quot;);
if (lintpos == -1)
    lintpos = top.document.title.length;
top.document.title= top.document.title.substr(0,lintpos) + &quot; / V&quot;;
function SetupToolBar(){ top.frames[&quot;fraHeader&quot;].pstrCodispl=&quot;CA003&quot;;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A301&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A302&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A303&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A304&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A310&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A401&quot;, false);
lblnQuery= (top.fraSequence.plngMainAction == 401 || top.fraSequence.pblnQuery) ;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A390&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A391&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A306&quot;, lblnQuery &amp;&amp;  false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A392&quot;, true);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A393&quot;, !lblnQuery);
top.fraHeader.insDisableHeader();
top.frames[&quot;fraHeader&quot;].setPointer('');}
function InvokeSetupToolBar(){try{SetupToolBar();}
catch(x){setTimeout('InvokeSetupToolBar()',150);}
finally{}} 
InvokeSetupToolBar();



    
    mstrType_Debit=&quot;2&quot;;mintDay=&quot;5&quot;;mstrInd=&quot;&quot;;
    
        
            
                
                    Tipo
            
            
                 
            
        
        
            
            
            
            
        
        
            
                Pago a través de banco
            
            
                 
            
        
        
            
                Pago con tarjeta de crédito
            
            
                 
            
            
                
                    Día de pago
            
            
                var mstrDoSubmit = &quot;1&quot;;if (window.event.keyCode==32)window.event.keyCode=8;
            
        
        
        
            
                
                    Datos básicos
            
        
        
            
            
        
        
            
                
                    Titular
            
            
                var mintTypeForm=2;Allo , Camdela
            
        
        
            
                
                    Pago a través de banco
            
        
        
            
            
        
        
            
                
                    Banco
            
            
                mstrCLient = '00000042933160';


if (window.event.keyCode==32)window.event.keyCode=8; document.btncbeBankExt.disabled=true



document.forms[0].cbeBankExt.CanShowValues=true

var Parameters_cbeBankExt= new Object;
var RParameters_cbeBankExt= new Object;
document.forms[0].elements['cbeBankExt'].TypeList='0';
document.forms[0].elements['cbeBankExt'].List='';
document.forms[0].elements['cbeBankExt'].TypeOrder='2';
RParameters_cbeBankExt.nCount=0;
document.forms[0].elements['cbeBankExt'].RParameters =RParameters_cbeBankExt;
Parameters_cbeBankExt.nCount=0;
document.forms[0].elements['cbeBankExt'].Parameters =Parameters_cbeBankExt;
document.forms[0].elements['cbeBankExt'].sTabName='table7';
document.forms[0].elements['cbeBankExt'].LookupAssembly='';
document.forms[0].elements['cbeBankExt'].LookupClass='';
SetParameters(document.forms[0].elements['cbeBankExt']);


            
            
                 
            
            
                
                    Cuenta
            
                


 



document.forms[0].valAccount.CanShowValues=true

var Parameters_valAccount= new Object;
 var valAccount1= new Object;
valAccount1.sName='sClient';
valAccount1.sValue='00000042933160';
valAccount1.sDirection='1';
valAccount1.sParType='22';
valAccount1.sSize='14';
valAccount1.sNumericScale='0';
valAccount1.sPrecision='0';
valAccount1.sAttributes='64';
Parameters_valAccount.Param1=valAccount1;
 var valAccount2= new Object;
valAccount2.sName='nBankExt';
valAccount2.sValue='-32768,3276';
valAccount2.sDirection='1';
valAccount2.sParType='6';
valAccount2.sSize='22';
valAccount2.sNumericScale='0';
valAccount2.sPrecision='10';
valAccount2.sAttributes='64';
Parameters_valAccount.Param2=valAccount2;
 var valAccount3= new Object;
valAccount3.sName='nCard_type';
valAccount3.sValue='0';
valAccount3.sDirection='1';
valAccount3.sParType='6';
valAccount3.sSize='22';
valAccount3.sNumericScale='0';
valAccount3.sPrecision='10';
valAccount3.sAttributes='64';
Parameters_valAccount.Param3=valAccount3;
var RParameters_valAccount= new Object;
document.forms[0].elements['valAccount'].TypeList='0';
document.forms[0].elements['valAccount'].List='';
document.forms[0].elements['valAccount'].TypeOrder='2';
RParameters_valAccount.nCount=0;
document.forms[0].elements['valAccount'].RParameters =RParameters_valAccount;
Parameters_valAccount.nCount=3;
document.forms[0].elements['valAccount'].Parameters =Parameters_valAccount;
document.forms[0].elements['valAccount'].sTabName='tabbk_account';
document.forms[0].elements['valAccount'].LookupAssembly='';
document.forms[0].elements['valAccount'].LookupClass='';
SetParameters(document.forms[0].elements['valAccount']);


            
        
        
            
                
                    Tipo de cuenta
            
            
                Caja cheque a fecha (Reservad)Caja cheques (Reservado)Caja efectivo (Reservado)Cuenta CorrienteCuenta de ahorroFondo de activos líquidosNo aplicaTarjeta de Credito(Reservado)
            
            
                 
            
            
                
                    Nro. de mandato
            
            
                
            
        
         
            
                
                    Moneda
            
            
                Dolar DivisaDólaresPesosSalario E.Adm A Conv.MercantilSalario Mínimo Vital y MóvilSalario Vida Obligatorio
            
            
                 
            
            
                  
            
            
                  
            
        
        
            
                 
            
        
         
            
                
                    Pago a través de CBU
            
        
        
            
            
        
        
        
                
             CBU
        
        
              
          
              
        
        
            
                
                    Pago con tarjeta de crédito
            
        
        
            
            
        
        
            
                
                    Tipo
            
            
                American ExpressCredialDinersMastercardNo InformadaSUCREDITOTarjeta AMEX PCITarjeta CabalTarjeta Cabal PCITarjeta GruparTarjeta Master PCITarjeta NaranjaTarjeta Naranja PCITarjeta NevadaTarjeta VisaTU ENTRADA AMERICAN EXPRESSTU ENTRADA CABALTU ENTRADA MASTERTU ENTRADA VISATU ENTRADA VISA DEBITOVISA PCI
            
            
                 
            
            
                
                    Número
            
                


 



document.forms[0].valcredi_card.CanShowValues=true

var Parameters_valcredi_card= new Object;
 var valcredi_card1= new Object;
valcredi_card1.sName='sClient';
valcredi_card1.sValue='00000042933160';
valcredi_card1.sDirection='1';
valcredi_card1.sParType='22';
valcredi_card1.sSize='14';
valcredi_card1.sNumericScale='0';
valcredi_card1.sPrecision='0';
valcredi_card1.sAttributes='64';
Parameters_valcredi_card.Param1=valcredi_card1;
 var valcredi_card2= new Object;
valcredi_card2.sName='nBankExt';
valcredi_card2.sValue='0';
valcredi_card2.sDirection='1';
valcredi_card2.sParType='6';
valcredi_card2.sSize='22';
valcredi_card2.sNumericScale='0';
valcredi_card2.sPrecision='10';
valcredi_card2.sAttributes='64';
Parameters_valcredi_card.Param2=valcredi_card2;
 var valcredi_card3= new Object;
valcredi_card3.sName='nCard_type';
valcredi_card3.sValue='16';
valcredi_card3.sDirection='1';
valcredi_card3.sParType='6';
valcredi_card3.sSize='22';
valcredi_card3.sNumericScale='0';
valcredi_card3.sPrecision='10';
valcredi_card3.sAttributes='64';
Parameters_valcredi_card.Param3=valcredi_card3;
var RParameters_valcredi_card= new Object;
document.forms[0].elements['valcredi_card'].TypeList='0';
document.forms[0].elements['valcredi_card'].List='';
document.forms[0].elements['valcredi_card'].TypeOrder='2';
RParameters_valcredi_card.nCount=0;
document.forms[0].elements['valcredi_card'].RParameters =RParameters_valcredi_card;
Parameters_valcredi_card.nCount=3;
document.forms[0].elements['valcredi_card'].Parameters =Parameters_valcredi_card;
document.forms[0].elements['valcredi_card'].sTabName='tabcred_card';
document.forms[0].elements['valcredi_card'].LookupAssembly='';
document.forms[0].elements['valcredi_card'].LookupClass='';
SetParameters(document.forms[0].elements['valcredi_card']);


            
        
        
            
                
                    Fecha de vencimiento
            
            
                
            
        
    
    $(document).ready(insChangeType_Debit);
    


/html[1]/body[1]</value>
      <webElementGuid>2e408b93-d394-440e-9d65-1d01ea0e6987</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>8066b5db-b1a1-4ff2-899e-20628574b083</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_Tratamiento de plizas (Company ABC)  V/frame_BODY            PEsta pgina utiliza f_a3efde</value>
      <webElementGuid>1c339de6-b8f7-4b98-b6b6-81c6d3890976</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>28211013-6aef-40b2-bbc9-23b3fbd97cbe</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;Vía de cobro    mstrSrvDecSep = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;; mstrUsrDecSep = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;; top.frames[&quot;fraSequence&quot;].pintZone=2;
var lblnQuery;
var lintpos;
lintpos = top.document.title.search(&quot; / &quot;);
if (lintpos == -1)
    lintpos = top.document.title.length;
top.document.title= top.document.title.substr(0,lintpos) + &quot; / V&quot;;
function SetupToolBar(){ top.frames[&quot;fraHeader&quot;].pstrCodispl=&quot;CA003&quot;;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A301&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A302&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A303&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A304&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A310&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A401&quot;, false);
lblnQuery= (top.fraSequence.plngMainAction == 401 || top.fraSequence.pblnQuery) ;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A390&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A391&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A306&quot;, lblnQuery &amp;&amp;  false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A392&quot;, true);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A393&quot;, !lblnQuery);
top.fraHeader.insDisableHeader();
top.frames[&quot;fraHeader&quot;].setPointer(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);}
function InvokeSetupToolBar(){try{SetupToolBar();}
catch(x){setTimeout(&quot; , &quot;'&quot; , &quot;InvokeSetupToolBar()&quot; , &quot;'&quot; , &quot;,150);}
finally{}} 
InvokeSetupToolBar();



    
    mstrType_Debit=&quot;2&quot;;mintDay=&quot;5&quot;;mstrInd=&quot;&quot;;
    
        
            
                
                    Tipo
            
            
                 
            
        
        
            
            
            
            
        
        
            
                Pago a través de banco
            
            
                 
            
        
        
            
                Pago con tarjeta de crédito
            
            
                 
            
            
                
                    Día de pago
            
            
                var mstrDoSubmit = &quot;1&quot;;if (window.event.keyCode==32)window.event.keyCode=8;
            
        
        
        
            
                
                    Datos básicos
            
        
        
            
            
        
        
            
                
                    Titular
            
            
                var mintTypeForm=2;Allo , Camdela
            
        
        
            
                
                    Pago a través de banco
            
        
        
            
            
        
        
            
                
                    Banco
            
            
                mstrCLient = &quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;


if (window.event.keyCode==32)window.event.keyCode=8; document.btncbeBankExt.disabled=true



document.forms[0].cbeBankExt.CanShowValues=true

var Parameters_cbeBankExt= new Object;
var RParameters_cbeBankExt= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_cbeBankExt.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeBankExt;
Parameters_cbeBankExt.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeBankExt;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;table7&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;]);


            
            
                 
            
            
                
                    Cuenta
            
                


 



document.forms[0].valAccount.CanShowValues=true

var Parameters_valAccount= new Object;
 var valAccount1= new Object;
valAccount1.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valAccount1.sValue=&quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;
valAccount1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount1.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valAccount1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param1=valAccount1;
 var valAccount2= new Object;
valAccount2.sName=&quot; , &quot;'&quot; , &quot;nBankExt&quot; , &quot;'&quot; , &quot;;
valAccount2.sValue=&quot; , &quot;'&quot; , &quot;-32768,3276&quot; , &quot;'&quot; , &quot;;
valAccount2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valAccount2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valAccount2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param2=valAccount2;
 var valAccount3= new Object;
valAccount3.sName=&quot; , &quot;'&quot; , &quot;nCard_type&quot; , &quot;'&quot; , &quot;;
valAccount3.sValue=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valAccount3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valAccount3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param3=valAccount3;
var RParameters_valAccount= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_valAccount.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valAccount;
Parameters_valAccount.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valAccount;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabbk_account&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;]);


            
        
        
            
                
                    Tipo de cuenta
            
            
                Caja cheque a fecha (Reservad)Caja cheques (Reservado)Caja efectivo (Reservado)Cuenta CorrienteCuenta de ahorroFondo de activos líquidosNo aplicaTarjeta de Credito(Reservado)
            
            
                 
            
            
                
                    Nro. de mandato
            
            
                
            
        
         
            
                
                    Moneda
            
            
                Dolar DivisaDólaresPesosSalario E.Adm A Conv.MercantilSalario Mínimo Vital y MóvilSalario Vida Obligatorio
            
            
                 
            
            
                  
            
            
                  
            
        
        
            
                 
            
        
         
            
                
                    Pago a través de CBU
            
        
        
            
            
        
        
        
                
             CBU
        
        
              
          
              
        
        
            
                
                    Pago con tarjeta de crédito
            
        
        
            
            
        
        
            
                
                    Tipo
            
            
                American ExpressCredialDinersMastercardNo InformadaSUCREDITOTarjeta AMEX PCITarjeta CabalTarjeta Cabal PCITarjeta GruparTarjeta Master PCITarjeta NaranjaTarjeta Naranja PCITarjeta NevadaTarjeta VisaTU ENTRADA AMERICAN EXPRESSTU ENTRADA CABALTU ENTRADA MASTERTU ENTRADA VISATU ENTRADA VISA DEBITOVISA PCI
            
            
                 
            
            
                
                    Número
            
                


 



document.forms[0].valcredi_card.CanShowValues=true

var Parameters_valcredi_card= new Object;
 var valcredi_card1= new Object;
valcredi_card1.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sValue=&quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param1=valcredi_card1;
 var valcredi_card2= new Object;
valcredi_card2.sName=&quot; , &quot;'&quot; , &quot;nBankExt&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sValue=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param2=valcredi_card2;
 var valcredi_card3= new Object;
valcredi_card3.sName=&quot; , &quot;'&quot; , &quot;nCard_type&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sValue=&quot; , &quot;'&quot; , &quot;16&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param3=valcredi_card3;
var RParameters_valcredi_card= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_valcredi_card.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valcredi_card;
Parameters_valcredi_card.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valcredi_card;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabcred_card&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;]);


            
        
        
            
                
                    Fecha de vencimiento
            
            
                
            
        
    
    $(document).ready(insChangeType_Debit);
    


/html[1]/body[1]&quot;) or . = concat(&quot;Vía de cobro    mstrSrvDecSep = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;; mstrUsrDecSep = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;; top.frames[&quot;fraSequence&quot;].pintZone=2;
var lblnQuery;
var lintpos;
lintpos = top.document.title.search(&quot; / &quot;);
if (lintpos == -1)
    lintpos = top.document.title.length;
top.document.title= top.document.title.substr(0,lintpos) + &quot; / V&quot;;
function SetupToolBar(){ top.frames[&quot;fraHeader&quot;].pstrCodispl=&quot;CA003&quot;;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A301&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A302&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A303&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A304&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A310&quot;, false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A401&quot;, false);
lblnQuery= (top.fraSequence.plngMainAction == 401 || top.fraSequence.pblnQuery) ;
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A390&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A391&quot;, !lblnQuery);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A306&quot;, lblnQuery &amp;&amp;  false);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A392&quot;, true);
top.frames[&quot;fraHeader&quot;].insHandImage(&quot;A393&quot;, !lblnQuery);
top.fraHeader.insDisableHeader();
top.frames[&quot;fraHeader&quot;].setPointer(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);}
function InvokeSetupToolBar(){try{SetupToolBar();}
catch(x){setTimeout(&quot; , &quot;'&quot; , &quot;InvokeSetupToolBar()&quot; , &quot;'&quot; , &quot;,150);}
finally{}} 
InvokeSetupToolBar();



    
    mstrType_Debit=&quot;2&quot;;mintDay=&quot;5&quot;;mstrInd=&quot;&quot;;
    
        
            
                
                    Tipo
            
            
                 
            
        
        
            
            
            
            
        
        
            
                Pago a través de banco
            
            
                 
            
        
        
            
                Pago con tarjeta de crédito
            
            
                 
            
            
                
                    Día de pago
            
            
                var mstrDoSubmit = &quot;1&quot;;if (window.event.keyCode==32)window.event.keyCode=8;
            
        
        
        
            
                
                    Datos básicos
            
        
        
            
            
        
        
            
                
                    Titular
            
            
                var mintTypeForm=2;Allo , Camdela
            
        
        
            
                
                    Pago a través de banco
            
        
        
            
            
        
        
            
                
                    Banco
            
            
                mstrCLient = &quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;


if (window.event.keyCode==32)window.event.keyCode=8; document.btncbeBankExt.disabled=true



document.forms[0].cbeBankExt.CanShowValues=true

var Parameters_cbeBankExt= new Object;
var RParameters_cbeBankExt= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_cbeBankExt.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeBankExt;
Parameters_cbeBankExt.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeBankExt;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;table7&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeBankExt&quot; , &quot;'&quot; , &quot;]);


            
            
                 
            
            
                
                    Cuenta
            
                


 



document.forms[0].valAccount.CanShowValues=true

var Parameters_valAccount= new Object;
 var valAccount1= new Object;
valAccount1.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valAccount1.sValue=&quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;
valAccount1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount1.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valAccount1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param1=valAccount1;
 var valAccount2= new Object;
valAccount2.sName=&quot; , &quot;'&quot; , &quot;nBankExt&quot; , &quot;'&quot; , &quot;;
valAccount2.sValue=&quot; , &quot;'&quot; , &quot;-32768,3276&quot; , &quot;'&quot; , &quot;;
valAccount2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valAccount2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valAccount2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param2=valAccount2;
 var valAccount3= new Object;
valAccount3.sName=&quot; , &quot;'&quot; , &quot;nCard_type&quot; , &quot;'&quot; , &quot;;
valAccount3.sValue=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valAccount3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valAccount3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valAccount3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valAccount3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valAccount3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valAccount.Param3=valAccount3;
var RParameters_valAccount= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_valAccount.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valAccount;
Parameters_valAccount.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valAccount;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabbk_account&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valAccount&quot; , &quot;'&quot; , &quot;]);


            
        
        
            
                
                    Tipo de cuenta
            
            
                Caja cheque a fecha (Reservad)Caja cheques (Reservado)Caja efectivo (Reservado)Cuenta CorrienteCuenta de ahorroFondo de activos líquidosNo aplicaTarjeta de Credito(Reservado)
            
            
                 
            
            
                
                    Nro. de mandato
            
            
                
            
        
         
            
                
                    Moneda
            
            
                Dolar DivisaDólaresPesosSalario E.Adm A Conv.MercantilSalario Mínimo Vital y MóvilSalario Vida Obligatorio
            
            
                 
            
            
                  
            
            
                  
            
        
        
            
                 
            
        
         
            
                
                    Pago a través de CBU
            
        
        
            
            
        
        
        
                
             CBU
        
        
              
          
              
        
        
            
                
                    Pago con tarjeta de crédito
            
        
        
            
            
        
        
            
                
                    Tipo
            
            
                American ExpressCredialDinersMastercardNo InformadaSUCREDITOTarjeta AMEX PCITarjeta CabalTarjeta Cabal PCITarjeta GruparTarjeta Master PCITarjeta NaranjaTarjeta Naranja PCITarjeta NevadaTarjeta VisaTU ENTRADA AMERICAN EXPRESSTU ENTRADA CABALTU ENTRADA MASTERTU ENTRADA VISATU ENTRADA VISA DEBITOVISA PCI
            
            
                 
            
            
                
                    Número
            
                


 



document.forms[0].valcredi_card.CanShowValues=true

var Parameters_valcredi_card= new Object;
 var valcredi_card1= new Object;
valcredi_card1.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sValue=&quot; , &quot;'&quot; , &quot;00000042933160&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param1=valcredi_card1;
 var valcredi_card2= new Object;
valcredi_card2.sName=&quot; , &quot;'&quot; , &quot;nBankExt&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sValue=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valcredi_card2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param2=valcredi_card2;
 var valcredi_card3= new Object;
valcredi_card3.sName=&quot; , &quot;'&quot; , &quot;nCard_type&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sValue=&quot; , &quot;'&quot; , &quot;16&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valcredi_card3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valcredi_card.Param3=valcredi_card3;
var RParameters_valcredi_card= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;;
RParameters_valcredi_card.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valcredi_card;
Parameters_valcredi_card.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valcredi_card;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabcred_card&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valcredi_card&quot; , &quot;'&quot; , &quot;]);


            
        
        
            
                
                    Fecha de vencimiento
            
            
                
            
        
    
    $(document).ready(insChangeType_Debit);
    


/html[1]/body[1]&quot;))]</value>
      <webElementGuid>4371c08f-d75b-44e9-a82d-096865bf1be9</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
