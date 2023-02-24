<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_AsegurablesClientesdelapliza var marrA_d4f3f2</name>
   <tag></tag>
   <elementGuidId>dfb48087-47e7-47f7-a21f-dc57750c2018</elementGuidId>
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
      <webElementGuid>132f3f1e-9e88-4cbc-a774-5af0eae4a4a8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onunload</name>
      <type>Main</type>
      <value>closeWindows();</value>
      <webElementGuid>130ea3e5-c3f5-4421-b7cd-ae2423d51986</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    Asegurables / Clientes de la póliza var marrArray = new Array(0); var mintArrayCount= -1;
function MarkRecord(Field){marrArray[Field.value].Sel = Field.checked}
function DeleteRecord(BeginIndex){
  var lintIndex=0; 
  for(lintIndex=(BeginIndex+1);(lintIndex&lt;=mintArrayCount) &amp;&amp; (!marrArray[lintIndex].Sel);lintIndex++){}
  if (lintIndex&lt;=mintArrayCount) EditRecord(lintIndex,nMainAction,'Del','sCode=' + marrArray[lintIndex].tctCode + '&amp;nCoverPos=' + marrArray[lintIndex].hddnCoverPos + '&amp;nRole=' + marrArray[lintIndex].cbeRole + '&amp;sRequire=' + marrArray[lintIndex].hddsRequire + '&amp;sInterClient=' + marrArray[lintIndex].hddsInterClient  +  '&amp;sCodisp=CA025&amp;sWindowDescript=Asegurables&amp;nWindowTy=9')
}
function insAddRecordArray(Sel,btnQuery,cbeRoleVal,cbeRole,tctCode,tctCode_Digit,lblCliename,cbeTypenameVal,cbeTypename,tctPrintName,cbeStatusrol,hddsOldCode,hddnMaxRole,hddnExist,hddsOldRole,hddnCoverPos,hddsRequire,hddsInterClient){
 var lobjElem = new Object; lobjElem.Sel = Sel;
 lobjElem.btnQuery = btnQuery;
 lobjElem.cbeRoleVal = cbeRoleVal;
 lobjElem.cbeRole = cbeRole;
 lobjElem.tctCode = tctCode;
 lobjElem.tctCode_Digit = tctCode_Digit;
 lobjElem.lblCliename = lblCliename;
 lobjElem.cbeTypenameVal = cbeTypenameVal;
 lobjElem.cbeTypename = cbeTypename;
 lobjElem.tctPrintName = tctPrintName;
 lobjElem.cbeStatusrol = cbeStatusrol;
 lobjElem.hddsOldCode = hddsOldCode;
 lobjElem.hddnMaxRole = hddnMaxRole;
 lobjElem.hddnExist = hddnExist;
 lobjElem.hddsOldRole = hddsOldRole;
 lobjElem.hddnCoverPos = hddnCoverPos;
 lobjElem.hddsRequire = hddsRequire;
 lobjElem.hddsInterClient = hddsInterClient;
 marrArray[++mintArrayCount] = lobjElem
}
function EditRecord(Field, nMainAction,Action,Param){ 
 if (typeof(Action)=='undefined') Action='Update';
 if (typeof(Param)=='undefined'){Param=''} 
 else {Param=(Param==''?'':'&amp;' + Param)};
 ShowPopUp(&quot;/VTimeNet/Common/EditRecord.aspx?Type=PopUp&amp;Action=&quot; + Action + &quot;&amp;Index=&quot; + Field + &quot;&amp;nMainAction=&quot; + nMainAction + &quot;&amp;sCodispl=CA025&quot; + Param,&quot;CA025Upd&quot;, (Action=='Del'?370:630), (Action=='Del'?130:295),'no','no',100,(Action=='Del'?200:100));}

 
SelInfoFiguraCódigo del clienteNombre a imprimirRazón Social a imprimir
 

Contratante
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;&quot;);


Asegurado
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;2&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;2&quot;,&quot;2&quot;,&quot;1&quot;,&quot;&quot;);


Intermediario
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;13&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;10&quot;,&quot;2&quot;,&quot;13&quot;,&quot;&quot;,&quot;2&quot;,&quot;&quot;);


Ejecutivo comercial
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;65&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;1&quot;,&quot;2&quot;,&quot;65&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;);

    



    //%InsChangeSel: Se envía mensaje de validación al no poder eliminar un registro
    //------------------------------------------------------------------------------
    function InsChangeSel(Field, sInd) {
        //------------------------------------------------------------------------------

        if (Field.checked &amp;&amp; sInd == &quot;2&quot;) {
            if (Brancht == &quot;1&quot; &amp;&amp; (Transac == '12' || Transac == '26')) {
                alert('Err. 56000 No se puede eliminar/modificar el &quot;Asegurado Principal&quot;');
                Field.checked = false
            }
        }
    }

/html[1]/body[1]</value>
      <webElementGuid>acb03bfd-6f52-420a-a8a3-e924901b97d1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>78de0d26-cc99-45e3-902d-be10e929b9b6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/frame_BODY            PEsta pgina utiliza f_a3efde</value>
      <webElementGuid>c3dc9cee-a884-4ccb-8993-6bf26c663dd9</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>44638e2f-85e0-4c8f-a6f1-cc7fa4e5b33a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
    Asegurables / Clientes de la póliza var marrArray = new Array(0); var mintArrayCount= -1;
function MarkRecord(Field){marrArray[Field.value].Sel = Field.checked}
function DeleteRecord(BeginIndex){
  var lintIndex=0; 
  for(lintIndex=(BeginIndex+1);(lintIndex&lt;=mintArrayCount) &amp;&amp; (!marrArray[lintIndex].Sel);lintIndex++){}
  if (lintIndex&lt;=mintArrayCount) EditRecord(lintIndex,nMainAction,&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;sCode=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].tctCode + &quot; , &quot;'&quot; , &quot;&amp;nCoverPos=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddnCoverPos + &quot; , &quot;'&quot; , &quot;&amp;nRole=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].cbeRole + &quot; , &quot;'&quot; , &quot;&amp;sRequire=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddsRequire + &quot; , &quot;'&quot; , &quot;&amp;sInterClient=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddsInterClient  +  &quot; , &quot;'&quot; , &quot;&amp;sCodisp=CA025&amp;sWindowDescript=Asegurables&amp;nWindowTy=9&quot; , &quot;'&quot; , &quot;)
}
function insAddRecordArray(Sel,btnQuery,cbeRoleVal,cbeRole,tctCode,tctCode_Digit,lblCliename,cbeTypenameVal,cbeTypename,tctPrintName,cbeStatusrol,hddsOldCode,hddnMaxRole,hddnExist,hddsOldRole,hddnCoverPos,hddsRequire,hddsInterClient){
 var lobjElem = new Object; lobjElem.Sel = Sel;
 lobjElem.btnQuery = btnQuery;
 lobjElem.cbeRoleVal = cbeRoleVal;
 lobjElem.cbeRole = cbeRole;
 lobjElem.tctCode = tctCode;
 lobjElem.tctCode_Digit = tctCode_Digit;
 lobjElem.lblCliename = lblCliename;
 lobjElem.cbeTypenameVal = cbeTypenameVal;
 lobjElem.cbeTypename = cbeTypename;
 lobjElem.tctPrintName = tctPrintName;
 lobjElem.cbeStatusrol = cbeStatusrol;
 lobjElem.hddsOldCode = hddsOldCode;
 lobjElem.hddnMaxRole = hddnMaxRole;
 lobjElem.hddnExist = hddnExist;
 lobjElem.hddsOldRole = hddsOldRole;
 lobjElem.hddnCoverPos = hddnCoverPos;
 lobjElem.hddsRequire = hddsRequire;
 lobjElem.hddsInterClient = hddsInterClient;
 marrArray[++mintArrayCount] = lobjElem
}
function EditRecord(Field, nMainAction,Action,Param){ 
 if (typeof(Action)==&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) Action=&quot; , &quot;'&quot; , &quot;Update&quot; , &quot;'&quot; , &quot;;
 if (typeof(Param)==&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){Param=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;} 
 else {Param=(Param==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; + Param)};
 ShowPopUp(&quot;/VTimeNet/Common/EditRecord.aspx?Type=PopUp&amp;Action=&quot; + Action + &quot;&amp;Index=&quot; + Field + &quot;&amp;nMainAction=&quot; + nMainAction + &quot;&amp;sCodispl=CA025&quot; + Param,&quot;CA025Upd&quot;, (Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?370:630), (Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?130:295),&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;,100,(Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?200:100));}

 
SelInfoFiguraCódigo del clienteNombre a imprimirRazón Social a imprimir
 

Contratante
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;&quot;);


Asegurado
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;2&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;2&quot;,&quot;2&quot;,&quot;1&quot;,&quot;&quot;);


Intermediario
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;13&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;10&quot;,&quot;2&quot;,&quot;13&quot;,&quot;&quot;,&quot;2&quot;,&quot;&quot;);


Ejecutivo comercial
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;65&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;1&quot;,&quot;2&quot;,&quot;65&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;);

    



    //%InsChangeSel: Se envía mensaje de validación al no poder eliminar un registro
    //------------------------------------------------------------------------------
    function InsChangeSel(Field, sInd) {
        //------------------------------------------------------------------------------

        if (Field.checked &amp;&amp; sInd == &quot;2&quot;) {
            if (Brancht == &quot;1&quot; &amp;&amp; (Transac == &quot; , &quot;'&quot; , &quot;12&quot; , &quot;'&quot; , &quot; || Transac == &quot; , &quot;'&quot; , &quot;26&quot; , &quot;'&quot; , &quot;)) {
                alert(&quot; , &quot;'&quot; , &quot;Err. 56000 No se puede eliminar/modificar el &quot;Asegurado Principal&quot;&quot; , &quot;'&quot; , &quot;);
                Field.checked = false
            }
        }
    }

/html[1]/body[1]&quot;) or . = concat(&quot;
    
    Asegurables / Clientes de la póliza var marrArray = new Array(0); var mintArrayCount= -1;
function MarkRecord(Field){marrArray[Field.value].Sel = Field.checked}
function DeleteRecord(BeginIndex){
  var lintIndex=0; 
  for(lintIndex=(BeginIndex+1);(lintIndex&lt;=mintArrayCount) &amp;&amp; (!marrArray[lintIndex].Sel);lintIndex++){}
  if (lintIndex&lt;=mintArrayCount) EditRecord(lintIndex,nMainAction,&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;sCode=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].tctCode + &quot; , &quot;'&quot; , &quot;&amp;nCoverPos=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddnCoverPos + &quot; , &quot;'&quot; , &quot;&amp;nRole=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].cbeRole + &quot; , &quot;'&quot; , &quot;&amp;sRequire=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddsRequire + &quot; , &quot;'&quot; , &quot;&amp;sInterClient=&quot; , &quot;'&quot; , &quot; + marrArray[lintIndex].hddsInterClient  +  &quot; , &quot;'&quot; , &quot;&amp;sCodisp=CA025&amp;sWindowDescript=Asegurables&amp;nWindowTy=9&quot; , &quot;'&quot; , &quot;)
}
function insAddRecordArray(Sel,btnQuery,cbeRoleVal,cbeRole,tctCode,tctCode_Digit,lblCliename,cbeTypenameVal,cbeTypename,tctPrintName,cbeStatusrol,hddsOldCode,hddnMaxRole,hddnExist,hddsOldRole,hddnCoverPos,hddsRequire,hddsInterClient){
 var lobjElem = new Object; lobjElem.Sel = Sel;
 lobjElem.btnQuery = btnQuery;
 lobjElem.cbeRoleVal = cbeRoleVal;
 lobjElem.cbeRole = cbeRole;
 lobjElem.tctCode = tctCode;
 lobjElem.tctCode_Digit = tctCode_Digit;
 lobjElem.lblCliename = lblCliename;
 lobjElem.cbeTypenameVal = cbeTypenameVal;
 lobjElem.cbeTypename = cbeTypename;
 lobjElem.tctPrintName = tctPrintName;
 lobjElem.cbeStatusrol = cbeStatusrol;
 lobjElem.hddsOldCode = hddsOldCode;
 lobjElem.hddnMaxRole = hddnMaxRole;
 lobjElem.hddnExist = hddnExist;
 lobjElem.hddsOldRole = hddsOldRole;
 lobjElem.hddnCoverPos = hddnCoverPos;
 lobjElem.hddsRequire = hddsRequire;
 lobjElem.hddsInterClient = hddsInterClient;
 marrArray[++mintArrayCount] = lobjElem
}
function EditRecord(Field, nMainAction,Action,Param){ 
 if (typeof(Action)==&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) Action=&quot; , &quot;'&quot; , &quot;Update&quot; , &quot;'&quot; , &quot;;
 if (typeof(Param)==&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){Param=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;} 
 else {Param=(Param==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; + Param)};
 ShowPopUp(&quot;/VTimeNet/Common/EditRecord.aspx?Type=PopUp&amp;Action=&quot; + Action + &quot;&amp;Index=&quot; + Field + &quot;&amp;nMainAction=&quot; + nMainAction + &quot;&amp;sCodispl=CA025&quot; + Param,&quot;CA025Upd&quot;, (Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?370:630), (Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?130:295),&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;,100,(Action==&quot; , &quot;'&quot; , &quot;Del&quot; , &quot;'&quot; , &quot;?200:100));}

 
SelInfoFiguraCódigo del clienteNombre a imprimirRazón Social a imprimir
 

Contratante
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;&quot;);


Asegurado
var mintTypeForm=1;00000042933160 -  Allo , Camdela

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;2&quot;,&quot;00000042933160&quot;,&quot;&quot;,&quot;Allo , Camdela&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;00000042933160&quot;,&quot;1&quot;,&quot;1&quot;,&quot;2&quot;,&quot;2&quot;,&quot;1&quot;,&quot;&quot;);


Intermediario
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;13&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;10&quot;,&quot;2&quot;,&quot;13&quot;,&quot;&quot;,&quot;2&quot;,&quot;&quot;);


Ejecutivo comercial
var mintTypeForm=1;

insAddRecordArray(false,&quot;&quot;,&quot;&quot;,&quot;65&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;,&quot;1&quot;,&quot;2&quot;,&quot;65&quot;,&quot;&quot;,&quot;1&quot;,&quot;&quot;);

    



    //%InsChangeSel: Se envía mensaje de validación al no poder eliminar un registro
    //------------------------------------------------------------------------------
    function InsChangeSel(Field, sInd) {
        //------------------------------------------------------------------------------

        if (Field.checked &amp;&amp; sInd == &quot;2&quot;) {
            if (Brancht == &quot;1&quot; &amp;&amp; (Transac == &quot; , &quot;'&quot; , &quot;12&quot; , &quot;'&quot; , &quot; || Transac == &quot; , &quot;'&quot; , &quot;26&quot; , &quot;'&quot; , &quot;)) {
                alert(&quot; , &quot;'&quot; , &quot;Err. 56000 No se puede eliminar/modificar el &quot;Asegurado Principal&quot;&quot; , &quot;'&quot; , &quot;);
                Field.checked = false
            }
        }
    }

/html[1]/body[1]&quot;))]</value>
      <webElementGuid>f9b1a25a-7687-46fe-88f2-aa3f805b1b16</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
