<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_MenuPrincipal</name>
   <tag></tag>
   <elementGuidId>cf293f20-0f69-419b-833e-2381d4db8e6e</elementGuidId>
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
      <webElementGuid>99d46603-9f31-4e52-971b-704fa0a3d47a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onunload</name>
      <type>Main</type>
      <value>closeWindows()</value>
      <webElementGuid>76e2580a-50df-4d06-872d-c16af8a98086</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Navegación Menú principal Transacción anterior Ir Salir del sistema Procesos batchAcciones Entrar Aceptar Cancelar Finalizar RefrescarAyuda Ayuda Ultimos valores Acerca de...

document.images['A208'].belongtoolbar=truedocument.images['A304'].belongtoolbar=truedocument.images['A390'].belongtoolbar=truedocument.images['A391'].belongtoolbar=truedocument.images['A392'].belongtoolbar=truedocument.images['A393'].belongtoolbar=true

if (top.frames[&quot;fraSequence&quot;].plngMainAction==0){
insHandImage(&quot;A208&quot;, true);
insHandImage(&quot;A301&quot;, true);
insHandImage(&quot;A302&quot;, true);
insHandImage(&quot;A303&quot;, true);
insHandImage(&quot;A304&quot;, true);
insHandImage(&quot;A305&quot;, true);
insHandImage(&quot;A306&quot;, true);
insHandImage(&quot;A310&quot;, true);
insHandImage(&quot;A401&quot;, true);
insHandImage(&quot;A402&quot;, true);
insHandImage(&quot;A390&quot;, false);
insHandImage(&quot;A391&quot;, false);
insHandImage(&quot;A392&quot;, false);
insHandImage(&quot;A393&quot;, false);
insHandImage(&quot;A394&quot;, true);
}
ClientRequest(304);
setPointer('');
top.document.title='Tratamiento de pólizas (Company ABC)';
var nMainAction=top.frames['fraSequence'].plngMainAction




    
    
    
    

//- Variable para indicar si se limpian campos
    var mblnCleanField

//- Variable qie contiene el tipo enumerado para identificar el tipo de compañía
    var eCompanyType = new eCompanyType()
    
//var eTypeAction  = new TypeAction()

//- Variable que almacena el tipo de la compañía usuaria
    var mstrCompanyType = '1'
    
//- Variable que almacena la oficina del usuario de tipo intermediario que maneja la página
    var mintUserOffice = ''
    
//- Variable que almacena la agencia del usuario de tipo intermediario que maneja la página
    var mintUserOfficeAgen = ''
    
//- Variable que almacena la sucursal del usuario de tipo intermediario que maneja la página
    var mintUserAgency = ''
    
// - Variable para definir el tipo de documento (póliza, Solicitud, Cotización)
    var mstrCertype

//-Indicador de carga de menu
  var mintMenu = '';
  

//-Descripciones del campo póliza según transaccion
    var mstrPolicyDescript1 = '&lt;LABEL>Póliza&lt;/LABEL>';
    var mstrPolicyDescript4 = '&lt;LABEL>Cotización&lt;/LABEL>';
    var mstrPolicyDescript6 = '&lt;LABEL>Propuesta&lt;/LABEL>';
    var mstrPolicyDescript43 = '&lt;LABEL>Propuesta de Rehabilitación&lt;/LABEL>';

//-Correspondencia Javascript de variables VbScript
    var mdtmEffecdate   = ''
    var mintBranch      = ''
    var mintProduct     = ''
    var mintPolicy      = ''
    var mintProponum    = ''
    var mintCertificat  = ''





    
        
                        
             Clave    
            
            
                  
            
                Transacción
                
                    Aprobar solicitud de endosoConsulta de CertificadosConsulta de CotizaciónConsulta de PólizaConsulta de PropuestaConsulta Prop RehabilitaciónConv.Cotiz.Modi. a Prop. Modi.Conv.Cotiz.Modif. a Modif Nor.Convertir Cotiz. Renov. a Pól.Convertir Cotización a PólizaConvertir Cotización a Prop.Convertir Prop.  Renov. a Pól.Convertir Propuesta a PólizaCotización de CertificadosCotización de endoso(Certif)Cotización de endoso(Póliza)Cotización de Renov. Certif.Cotización de Renov. de PólizaCotización PólizaDuplicar PólizaEmisión CertificadoEmisión de PólizaEndoso a la PólizaEndoso al CertificadoModificación Temporal de Cert.Modificación Temporal de Pól.Propuesta de Renov. Certif.RecuperaciónRe-Emision de CertificadoRe-Emision de PólizaRe-ImpresiónSolicitud de endoso (Certif.)Solicitud de endoso (Póliza)Solicitudes de Emis - CertifSolicitudes de Emis - PólSolictud de Renov. de PólizaTraspaso de asegurado
            
            
                Fecha de vigencia
                var mstrDoSubmit = &quot;1&quot;;
            
            
                Sucursal
                
                Casa Central
                
            
            
                Oficina
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeOfficeAgen.CanShowValues=true

var Parameters_cbeOfficeAgen= new Object;
 var cbeOfficeAgen1= new Object;
cbeOfficeAgen1.sName='nOfficeAgen';
cbeOfficeAgen1.sValue='-32768';
cbeOfficeAgen1.sDirection='1';
cbeOfficeAgen1.sParType='6';
cbeOfficeAgen1.sSize='22';
cbeOfficeAgen1.sNumericScale='0';
cbeOfficeAgen1.sPrecision='10';
cbeOfficeAgen1.sAttributes='64';
Parameters_cbeOfficeAgen.Param1=cbeOfficeAgen1;
 var cbeOfficeAgen2= new Object;
cbeOfficeAgen2.sName='nAgency';
cbeOfficeAgen2.sValue='-32768';
cbeOfficeAgen2.sDirection='1';
cbeOfficeAgen2.sParType='6';
cbeOfficeAgen2.sSize='22';
cbeOfficeAgen2.sNumericScale='0';
cbeOfficeAgen2.sPrecision='10';
cbeOfficeAgen2.sAttributes='64';
Parameters_cbeOfficeAgen.Param2=cbeOfficeAgen2;
var RParameters_cbeOfficeAgen= new Object;
 var RcbeOfficeAgen1= new Object;
RcbeOfficeAgen1.Name='nBran_off';
RcbeOfficeAgen1.Visible='False';
RcbeOfficeAgen1.Title='';
RcbeOfficeAgen1.Create='True';
RParameters_cbeOfficeAgen.Param1=RcbeOfficeAgen1;
document.forms[0].elements['cbeOfficeAgen'].TypeList='0';
document.forms[0].elements['cbeOfficeAgen'].List='';
document.forms[0].elements['cbeOfficeAgen'].TypeOrder='1';
RParameters_cbeOfficeAgen.nCount=1;
document.forms[0].elements['cbeOfficeAgen'].RParameters =RParameters_cbeOfficeAgen;
Parameters_cbeOfficeAgen.nCount=2;
document.forms[0].elements['cbeOfficeAgen'].Parameters =Parameters_cbeOfficeAgen;
document.forms[0].elements['cbeOfficeAgen'].sTabName='TabAgencies_T5556';
document.forms[0].elements['cbeOfficeAgen'].LookupAssembly='';
document.forms[0].elements['cbeOfficeAgen'].LookupClass='';
SetParameters(document.forms[0].elements['cbeOfficeAgen']);


                
            
            
                Agencia
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeAgency.CanShowValues=true

var Parameters_cbeAgency= new Object;
 var cbeAgency1= new Object;
cbeAgency1.sName='nOfficeAgen';
cbeAgency1.sValue='-32768';
cbeAgency1.sDirection='1';
cbeAgency1.sParType='6';
cbeAgency1.sSize='22';
cbeAgency1.sNumericScale='0';
cbeAgency1.sPrecision='10';
cbeAgency1.sAttributes='64';
Parameters_cbeAgency.Param1=cbeAgency1;
 var cbeAgency2= new Object;
cbeAgency2.sName='nAgency';
cbeAgency2.sValue='-32768';
cbeAgency2.sDirection='1';
cbeAgency2.sParType='6';
cbeAgency2.sSize='22';
cbeAgency2.sNumericScale='0';
cbeAgency2.sPrecision='10';
cbeAgency2.sAttributes='64';
Parameters_cbeAgency.Param2=cbeAgency2;
 var cbeAgency3= new Object;
cbeAgency3.sName='nUsercode';
cbeAgency3.sValue='9794';
cbeAgency3.sDirection='1';
cbeAgency3.sParType='6';
cbeAgency3.sSize='0';
cbeAgency3.sNumericScale='0';
cbeAgency3.sPrecision='5';
cbeAgency3.sAttributes='64';
Parameters_cbeAgency.Param3=cbeAgency3;
var RParameters_cbeAgency= new Object;
 var RcbeAgency1= new Object;
RcbeAgency1.Name='nBran_off';
RcbeAgency1.Visible='False';
RcbeAgency1.Title='';
RcbeAgency1.Create='True';
RParameters_cbeAgency.Param1=RcbeAgency1;
 var RcbeAgency2= new Object;
RcbeAgency2.Name='nOfficeAgen';
RcbeAgency2.Visible='False';
RcbeAgency2.Title='';
RcbeAgency2.Create='True';
RParameters_cbeAgency.Param2=RcbeAgency2;
 var RcbeAgency3= new Object;
RcbeAgency3.Name='sDesAgen';
RcbeAgency3.Visible='False';
RcbeAgency3.Title='';
RcbeAgency3.Create='True';
RParameters_cbeAgency.Param3=RcbeAgency3;
document.forms[0].elements['cbeAgency'].TypeList='0';
document.forms[0].elements['cbeAgency'].List='';
document.forms[0].elements['cbeAgency'].TypeOrder='1';
RParameters_cbeAgency.nCount=3;
document.forms[0].elements['cbeAgency'].RParameters =RParameters_cbeAgency;
Parameters_cbeAgency.nCount=3;
document.forms[0].elements['cbeAgency'].Parameters =Parameters_cbeAgency;
document.forms[0].elements['cbeAgency'].sTabName='TabAgencies_T5555A';
document.forms[0].elements['cbeAgency'].LookupAssembly='';
document.forms[0].elements['cbeAgency'].LookupClass='';
SetParameters(document.forms[0].elements['cbeAgency']);


                
                
            
            
                Canal de venta
                    
                     IntermediarioTELEMARKETINGDirectoWEBLICITACIÓNSucursales                    
                                                                        
            
                Ramo
                
function InsChangecbeBranch(nBranch){
    with(self.document.forms[0]){
        if (typeof(valProduct) != 'undefined'){
            valProduct.disabled=(nBranch=='0'||nBranch==''?true:false);
            btnvalProduct.disabled=valProduct.disabled;
            valProduct.value = '';
            UpdateDiv('valProductDesc', '');
            valProduct.Parameters.Param1.sValue = nBranch;
        }
    }
}

Comb. Familiar e IntegralesRobo y riesgos similaresVida colectivoRiesgos variosAccidentes PersonalesVida ObligatorioVida IndividualSaludSepelioGanado 
             
            
                Producto
                


if (window.event.keyCode==32)window.event.keyCode=8; document.btnvalProduct.disabled=true
Proteccion Integral


document.forms[0].valProduct.CanShowValues=true

var Parameters_valProduct= new Object;
 var valProduct1= new Object;
valProduct1.sName='nBranch';
valProduct1.sValue='-32768';
valProduct1.sDirection='1';
valProduct1.sParType='8';
valProduct1.sSize='0';
valProduct1.sNumericScale='0';
valProduct1.sPrecision='10';
valProduct1.sAttributes='64';
Parameters_valProduct.Param1=valProduct1;
var RParameters_valProduct= new Object;
document.forms[0].elements['valProduct'].TypeList='0';
document.forms[0].elements['valProduct'].List='';
document.forms[0].elements['valProduct'].TypeOrder='1';
RParameters_valProduct.nCount=0;
document.forms[0].elements['valProduct'].RParameters =RParameters_valProduct;
Parameters_valProduct.nCount=1;
document.forms[0].elements['valProduct'].Parameters =Parameters_valProduct;
document.forms[0].elements['valProduct'].sTabName='tabProdMaster1';
document.forms[0].elements['valProduct'].LookupAssembly='';
document.forms[0].elements['valProduct'].LookupClass='';
SetParameters(document.forms[0].elements['valProduct']);


            
            
                
                Póliza 

                  
                if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;
document.btntcnPolicy.disabled=true

self.document.forms[0].elements['tcnPolicy'].CertypeQuery=2;
function insShowPolicyQuerytcnPolicy(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp('/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=' + FieldName + '&amp;FieldBranch=' + FieldBranch + '&amp;FieldProduct=' + FieldProduct + '&amp;FieldCertif=' + FieldCertif + '&amp;nBranch=' + BranchValue + '&amp;nProduct=' + ProductValue + '&amp;TypeList=' + TypeList + '&amp;List=' + List + '&amp;sCertypeQuery=' + self.document.forms[0].tcnPolicy.CertypeQuery, 'ControldePoliza', 750 , 480, 'no', 'no', 20, 20)
}

				
            
            
				Póliza destino
			    
			    if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;


self.document.forms[0].elements['tcnPolicyDest'].CertypeQuery=2;
function insShowPolicyQuerytcnPolicyDest(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp('/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=' + FieldName + '&amp;FieldBranch=' + FieldBranch + '&amp;FieldProduct=' + FieldProduct + '&amp;FieldCertif=' + FieldCertif + '&amp;nBranch=' + BranchValue + '&amp;nProduct=' + ProductValue + '&amp;TypeList=' + TypeList + '&amp;List=' + List + '&amp;sCertypeQuery=' + self.document.forms[0].tcnPolicyDest.CertypeQuery, 'ControldePoliza', 750 , 480, 'no', 'no', 20, 20)
}

			
			
                Asegurado
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valCertif.CanShowValues=true

var Parameters_valCertif= new Object;
 var valCertif1= new Object;
valCertif1.sName='sCertype';
valCertif1.sValue='VT_EMPTY';
valCertif1.sDirection='1';
valCertif1.sParType='22';
valCertif1.sSize='1';
valCertif1.sNumericScale='0';
valCertif1.sPrecision='0';
valCertif1.sAttributes='64';
Parameters_valCertif.Param1=valCertif1;
 var valCertif2= new Object;
valCertif2.sName='nBranch';
valCertif2.sValue='VT_EMPTY';
valCertif2.sDirection='1';
valCertif2.sParType='6';
valCertif2.sSize='22';
valCertif2.sNumericScale='0';
valCertif2.sPrecision='10';
valCertif2.sAttributes='64';
Parameters_valCertif.Param2=valCertif2;
 var valCertif3= new Object;
valCertif3.sName='nProduct';
valCertif3.sValue='VT_EMPTY';
valCertif3.sDirection='1';
valCertif3.sParType='6';
valCertif3.sSize='22';
valCertif3.sNumericScale='0';
valCertif3.sPrecision='10';
valCertif3.sAttributes='64';
Parameters_valCertif.Param3=valCertif3;
 var valCertif4= new Object;
valCertif4.sName='nPolicy';
valCertif4.sValue='VT_EMPTY';
valCertif4.sDirection='1';
valCertif4.sParType='6';
valCertif4.sSize='22';
valCertif4.sNumericScale='0';
valCertif4.sPrecision='10';
valCertif4.sAttributes='64';
Parameters_valCertif.Param4=valCertif4;
 var valCertif5= new Object;
valCertif5.sName='sClient';
valCertif5.sValue='VT_EMPTY';
valCertif5.sDirection='1';
valCertif5.sParType='22';
valCertif5.sSize='14';
valCertif5.sNumericScale='0';
valCertif5.sPrecision='0';
valCertif5.sAttributes='64';
Parameters_valCertif.Param5=valCertif5;
 var valCertif6= new Object;
valCertif6.sName='dEffecdate';
valCertif6.sValue='23/02/2023';
valCertif6.sDirection='1';
valCertif6.sParType='19';
valCertif6.sSize='0';
valCertif6.sNumericScale='0';
valCertif6.sPrecision='0';
valCertif6.sAttributes='64';
Parameters_valCertif.Param6=valCertif6;
var RParameters_valCertif= new Object;
 var RvalCertif1= new Object;
RvalCertif1.Name='nCertif';
RvalCertif1.Visible='True';
RvalCertif1.Title='Certificado';
RvalCertif1.Create='True';
RParameters_valCertif.Param1=RvalCertif1;
document.forms[0].elements['valCertif'].TypeList='0';
document.forms[0].elements['valCertif'].List='';
document.forms[0].elements['valCertif'].TypeOrder='1';
RParameters_valCertif.nCount=1;
document.forms[0].elements['valCertif'].RParameters =RParameters_valCertif;
Parameters_valCertif.nCount=6;
document.forms[0].elements['valCertif'].Parameters =Parameters_valCertif;
document.forms[0].elements['valCertif'].sTabName='TabCertif';
document.forms[0].elements['valCertif'].LookupAssembly='';
document.forms[0].elements['valCertif'].LookupClass='';
SetParameters(document.forms[0].elements['valCertif']);


                
			
						    
                Certificado
                
                if (window.event.keyCode==32)window.event.keyCode=8;document.btnPolicyValues.disabled=true
                
            
            
                
                
                Cotización / Propuesta
                
                
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Propuesta regularizada
                if (window.event.keyCode==32)window.event.keyCode=8;
            
                
                Número de inspección
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Folio
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Número de renovaciones
                if (window.event.keyCode==32)window.event.keyCode=8;
            
        
    
     
    
        
                        
                Tipo de negocio            
            
            
                 
            
            
                
                    DirectoCoaseguro aceptadoReaseguro aceptado
                
            
            
                 
            
                        
                Tipo de póliza            
            
            
                 
              
            
                
                    IndividualColectivoMultilocalidad
                
            
            
                 
            
            
                
                    
                                    
                            Relaciones            
                        
                        
                            
                          
                        
                            Contabilización
                            
                        
                    
                
            
            
                                                
                                    
                        
                                        
                                Modificación            
                            
                            
                                        
                              
                            
                                Tipo de endoso
                                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valType_amend.CanShowValues=true

var Parameters_valType_amend= new Object;
 var valType_amend1= new Object;
valType_amend1.sName='nBranch';
valType_amend1.sValue='VT_EMPTY';
valType_amend1.sDirection='1';
valType_amend1.sParType='6';
valType_amend1.sSize='22';
valType_amend1.sNumericScale='0';
valType_amend1.sPrecision='10';
valType_amend1.sAttributes='64';
Parameters_valType_amend.Param1=valType_amend1;
 var valType_amend2= new Object;
valType_amend2.sName='nProduct';
valType_amend2.sValue='VT_EMPTY';
valType_amend2.sDirection='1';
valType_amend2.sParType='6';
valType_amend2.sSize='22';
valType_amend2.sNumericScale='0';
valType_amend2.sPrecision='10';
valType_amend2.sAttributes='64';
Parameters_valType_amend.Param2=valType_amend2;
 var valType_amend3= new Object;
valType_amend3.sName='dEffecdate';
valType_amend3.sValue='23/02/2023';
valType_amend3.sDirection='1';
valType_amend3.sParType='19';
valType_amend3.sSize='0';
valType_amend3.sNumericScale='0';
valType_amend3.sPrecision='0';
valType_amend3.sAttributes='64';
Parameters_valType_amend.Param3=valType_amend3;
var RParameters_valType_amend= new Object;
document.forms[0].elements['valType_amend'].TypeList='0';
document.forms[0].elements['valType_amend'].List='';
document.forms[0].elements['valType_amend'].TypeOrder='1';
RParameters_valType_amend.nCount=0;
document.forms[0].elements['valType_amend'].RParameters =RParameters_valType_amend;
Parameters_valType_amend.nCount=3;
document.forms[0].elements['valType_amend'].Parameters =Parameters_valType_amend;
document.forms[0].elements['valType_amend'].sTabName='Tabtype_amend';
document.forms[0].elements['valType_amend'].LookupAssembly='';
document.forms[0].elements['valType_amend'].LookupClass='';
SetParameters(document.forms[0].elements['valType_amend']);


                                
                            
                            
                                F.E.R.
                                
                            
                        
                    
                
            
        
    


    
        
            Afectar certificados
        
    


    


    
        
            
                            
                    Fecha de vencimiento            
                
                
                            
                                        
                
                    Fecha
                    
                
            
        
    

insInitialFields(true,'')
    


/html[1]/body[1]</value>
      <webElementGuid>95d59ed1-53dd-4954-a208-70193d819d32</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>560e7b2a-1f45-400b-9517-fa2d24544265</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_Tratamiento de plizas (Company ABC)/frame_BODY            PEsta pgina utiliza f_69e94c</value>
      <webElementGuid>9846c25c-2af7-40d0-84af-3ca4e02c5e58</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>f2b19164-e223-456d-a84e-ad8bc9674d1c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;Navegación Menú principal Transacción anterior Ir Salir del sistema Procesos batchAcciones Entrar Aceptar Cancelar Finalizar RefrescarAyuda Ayuda Ultimos valores Acerca de...

document.images[&quot; , &quot;'&quot; , &quot;A208&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A304&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A390&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A391&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A392&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A393&quot; , &quot;'&quot; , &quot;].belongtoolbar=true

if (top.frames[&quot;fraSequence&quot;].plngMainAction==0){
insHandImage(&quot;A208&quot;, true);
insHandImage(&quot;A301&quot;, true);
insHandImage(&quot;A302&quot;, true);
insHandImage(&quot;A303&quot;, true);
insHandImage(&quot;A304&quot;, true);
insHandImage(&quot;A305&quot;, true);
insHandImage(&quot;A306&quot;, true);
insHandImage(&quot;A310&quot;, true);
insHandImage(&quot;A401&quot;, true);
insHandImage(&quot;A402&quot;, true);
insHandImage(&quot;A390&quot;, false);
insHandImage(&quot;A391&quot;, false);
insHandImage(&quot;A392&quot;, false);
insHandImage(&quot;A393&quot;, false);
insHandImage(&quot;A394&quot;, true);
}
ClientRequest(304);
setPointer(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
top.document.title=&quot; , &quot;'&quot; , &quot;Tratamiento de pólizas (Company ABC)&quot; , &quot;'&quot; , &quot;;
var nMainAction=top.frames[&quot; , &quot;'&quot; , &quot;fraSequence&quot; , &quot;'&quot; , &quot;].plngMainAction




    
    
    
    

//- Variable para indicar si se limpian campos
    var mblnCleanField

//- Variable qie contiene el tipo enumerado para identificar el tipo de compañía
    var eCompanyType = new eCompanyType()
    
//var eTypeAction  = new TypeAction()

//- Variable que almacena el tipo de la compañía usuaria
    var mstrCompanyType = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la oficina del usuario de tipo intermediario que maneja la página
    var mintUserOffice = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la agencia del usuario de tipo intermediario que maneja la página
    var mintUserOfficeAgen = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la sucursal del usuario de tipo intermediario que maneja la página
    var mintUserAgency = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
// - Variable para definir el tipo de documento (póliza, Solicitud, Cotización)
    var mstrCertype

//-Indicador de carga de menu
  var mintMenu = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  

//-Descripciones del campo póliza según transaccion
    var mstrPolicyDescript1 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Póliza&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript4 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Cotización&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript6 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Propuesta&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript43 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Propuesta de Rehabilitación&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;

//-Correspondencia Javascript de variables VbScript
    var mdtmEffecdate   = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintBranch      = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintProduct     = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintPolicy      = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintProponum    = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintCertificat  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;





    
        
                        
             Clave    
            
            
                  
            
                Transacción
                
                    Aprobar solicitud de endosoConsulta de CertificadosConsulta de CotizaciónConsulta de PólizaConsulta de PropuestaConsulta Prop RehabilitaciónConv.Cotiz.Modi. a Prop. Modi.Conv.Cotiz.Modif. a Modif Nor.Convertir Cotiz. Renov. a Pól.Convertir Cotización a PólizaConvertir Cotización a Prop.Convertir Prop.  Renov. a Pól.Convertir Propuesta a PólizaCotización de CertificadosCotización de endoso(Certif)Cotización de endoso(Póliza)Cotización de Renov. Certif.Cotización de Renov. de PólizaCotización PólizaDuplicar PólizaEmisión CertificadoEmisión de PólizaEndoso a la PólizaEndoso al CertificadoModificación Temporal de Cert.Modificación Temporal de Pól.Propuesta de Renov. Certif.RecuperaciónRe-Emision de CertificadoRe-Emision de PólizaRe-ImpresiónSolicitud de endoso (Certif.)Solicitud de endoso (Póliza)Solicitudes de Emis - CertifSolicitudes de Emis - PólSolictud de Renov. de PólizaTraspaso de asegurado
            
            
                Fecha de vigencia
                var mstrDoSubmit = &quot;1&quot;;
            
            
                Sucursal
                
                Casa Central
                
            
            
                Oficina
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeOfficeAgen.CanShowValues=true

var Parameters_cbeOfficeAgen= new Object;
 var cbeOfficeAgen1= new Object;
cbeOfficeAgen1.sName=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeOfficeAgen.Param1=cbeOfficeAgen1;
 var cbeOfficeAgen2= new Object;
cbeOfficeAgen2.sName=&quot; , &quot;'&quot; , &quot;nAgency&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeOfficeAgen.Param2=cbeOfficeAgen2;
var RParameters_cbeOfficeAgen= new Object;
 var RcbeOfficeAgen1= new Object;
RcbeOfficeAgen1.Name=&quot; , &quot;'&quot; , &quot;nBran_off&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeOfficeAgen.Param1=RcbeOfficeAgen1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_cbeOfficeAgen.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeOfficeAgen;
Parameters_cbeOfficeAgen.nCount=2;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeOfficeAgen;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabAgencies_T5556&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;]);


                
            
            
                Agencia
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeAgency.CanShowValues=true

var Parameters_cbeAgency= new Object;
 var cbeAgency1= new Object;
cbeAgency1.sName=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param1=cbeAgency1;
 var cbeAgency2= new Object;
cbeAgency2.sName=&quot; , &quot;'&quot; , &quot;nAgency&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param2=cbeAgency2;
 var cbeAgency3= new Object;
cbeAgency3.sName=&quot; , &quot;'&quot; , &quot;nUsercode&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sValue=&quot; , &quot;'&quot; , &quot;9794&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sPrecision=&quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param3=cbeAgency3;
var RParameters_cbeAgency= new Object;
 var RcbeAgency1= new Object;
RcbeAgency1.Name=&quot; , &quot;'&quot; , &quot;nBran_off&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param1=RcbeAgency1;
 var RcbeAgency2= new Object;
RcbeAgency2.Name=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param2=RcbeAgency2;
 var RcbeAgency3= new Object;
RcbeAgency3.Name=&quot; , &quot;'&quot; , &quot;sDesAgen&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param3=RcbeAgency3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeAgency;
Parameters_cbeAgency.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeAgency;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabAgencies_T5555A&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;]);


                
                
            
            
                Canal de venta
                    
                     IntermediarioTELEMARKETINGDirectoWEBLICITACIÓNSucursales                    
                                                                        
            
                Ramo
                
function InsChangecbeBranch(nBranch){
    with(self.document.forms[0]){
        if (typeof(valProduct) != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){
            valProduct.disabled=(nBranch==&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;||nBranch==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;?true:false);
            btnvalProduct.disabled=valProduct.disabled;
            valProduct.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            UpdateDiv(&quot; , &quot;'&quot; , &quot;valProductDesc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            valProduct.Parameters.Param1.sValue = nBranch;
        }
    }
}

Comb. Familiar e IntegralesRobo y riesgos similaresVida colectivoRiesgos variosAccidentes PersonalesVida ObligatorioVida IndividualSaludSepelioGanado 
             
            
                Producto
                


if (window.event.keyCode==32)window.event.keyCode=8; document.btnvalProduct.disabled=true
Proteccion Integral


document.forms[0].valProduct.CanShowValues=true

var Parameters_valProduct= new Object;
 var valProduct1= new Object;
valProduct1.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valProduct1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
valProduct1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valProduct1.sParType=&quot; , &quot;'&quot; , &quot;8&quot; , &quot;'&quot; , &quot;;
valProduct1.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valProduct1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valProduct1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valProduct1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valProduct.Param1=valProduct1;
var RParameters_valProduct= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valProduct.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valProduct;
Parameters_valProduct.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valProduct;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabProdMaster1&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;]);


            
            
                
                Póliza 

                  
                if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;
document.btntcnPolicy.disabled=true

self.document.forms[0].elements[&quot; , &quot;'&quot; , &quot;tcnPolicy&quot; , &quot;'&quot; , &quot;].CertypeQuery=2;
function insShowPolicyQuerytcnPolicy(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp(&quot; , &quot;'&quot; , &quot;/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=&quot; , &quot;'&quot; , &quot; + FieldName + &quot; , &quot;'&quot; , &quot;&amp;FieldBranch=&quot; , &quot;'&quot; , &quot; + FieldBranch + &quot; , &quot;'&quot; , &quot;&amp;FieldProduct=&quot; , &quot;'&quot; , &quot; + FieldProduct + &quot; , &quot;'&quot; , &quot;&amp;FieldCertif=&quot; , &quot;'&quot; , &quot; + FieldCertif + &quot; , &quot;'&quot; , &quot;&amp;nBranch=&quot; , &quot;'&quot; , &quot; + BranchValue + &quot; , &quot;'&quot; , &quot;&amp;nProduct=&quot; , &quot;'&quot; , &quot; + ProductValue + &quot; , &quot;'&quot; , &quot;&amp;TypeList=&quot; , &quot;'&quot; , &quot; + TypeList + &quot; , &quot;'&quot; , &quot;&amp;List=&quot; , &quot;'&quot; , &quot; + List + &quot; , &quot;'&quot; , &quot;&amp;sCertypeQuery=&quot; , &quot;'&quot; , &quot; + self.document.forms[0].tcnPolicy.CertypeQuery, &quot; , &quot;'&quot; , &quot;ControldePoliza&quot; , &quot;'&quot; , &quot;, 750 , 480, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, 20, 20)
}

				
            
            
				Póliza destino
			    
			    if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;


self.document.forms[0].elements[&quot; , &quot;'&quot; , &quot;tcnPolicyDest&quot; , &quot;'&quot; , &quot;].CertypeQuery=2;
function insShowPolicyQuerytcnPolicyDest(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp(&quot; , &quot;'&quot; , &quot;/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=&quot; , &quot;'&quot; , &quot; + FieldName + &quot; , &quot;'&quot; , &quot;&amp;FieldBranch=&quot; , &quot;'&quot; , &quot; + FieldBranch + &quot; , &quot;'&quot; , &quot;&amp;FieldProduct=&quot; , &quot;'&quot; , &quot; + FieldProduct + &quot; , &quot;'&quot; , &quot;&amp;FieldCertif=&quot; , &quot;'&quot; , &quot; + FieldCertif + &quot; , &quot;'&quot; , &quot;&amp;nBranch=&quot; , &quot;'&quot; , &quot; + BranchValue + &quot; , &quot;'&quot; , &quot;&amp;nProduct=&quot; , &quot;'&quot; , &quot; + ProductValue + &quot; , &quot;'&quot; , &quot;&amp;TypeList=&quot; , &quot;'&quot; , &quot; + TypeList + &quot; , &quot;'&quot; , &quot;&amp;List=&quot; , &quot;'&quot; , &quot; + List + &quot; , &quot;'&quot; , &quot;&amp;sCertypeQuery=&quot; , &quot;'&quot; , &quot; + self.document.forms[0].tcnPolicyDest.CertypeQuery, &quot; , &quot;'&quot; , &quot;ControldePoliza&quot; , &quot;'&quot; , &quot;, 750 , 480, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, 20, 20)
}

			
			
                Asegurado
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valCertif.CanShowValues=true

var Parameters_valCertif= new Object;
 var valCertif1= new Object;
valCertif1.sName=&quot; , &quot;'&quot; , &quot;sCertype&quot; , &quot;'&quot; , &quot;;
valCertif1.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif1.sSize=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param1=valCertif1;
 var valCertif2= new Object;
valCertif2.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valCertif2.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param2=valCertif2;
 var valCertif3= new Object;
valCertif3.sName=&quot; , &quot;'&quot; , &quot;nProduct&quot; , &quot;'&quot; , &quot;;
valCertif3.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param3=valCertif3;
 var valCertif4= new Object;
valCertif4.sName=&quot; , &quot;'&quot; , &quot;nPolicy&quot; , &quot;'&quot; , &quot;;
valCertif4.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif4.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif4.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif4.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif4.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif4.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif4.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param4=valCertif4;
 var valCertif5= new Object;
valCertif5.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valCertif5.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif5.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif5.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif5.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valCertif5.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif5.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif5.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param5=valCertif5;
 var valCertif6= new Object;
valCertif6.sName=&quot; , &quot;'&quot; , &quot;dEffecdate&quot; , &quot;'&quot; , &quot;;
valCertif6.sValue=&quot; , &quot;'&quot; , &quot;23/02/2023&quot; , &quot;'&quot; , &quot;;
valCertif6.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif6.sParType=&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;;
valCertif6.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param6=valCertif6;
var RParameters_valCertif= new Object;
 var RvalCertif1= new Object;
RvalCertif1.Name=&quot; , &quot;'&quot; , &quot;nCertif&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Visible=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Title=&quot; , &quot;'&quot; , &quot;Certificado&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_valCertif.Param1=RvalCertif1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valCertif.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valCertif;
Parameters_valCertif.nCount=6;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valCertif;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabCertif&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;]);


                
			
						    
                Certificado
                
                if (window.event.keyCode==32)window.event.keyCode=8;document.btnPolicyValues.disabled=true
                
            
            
                
                
                Cotización / Propuesta
                
                
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Propuesta regularizada
                if (window.event.keyCode==32)window.event.keyCode=8;
            
                
                Número de inspección
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Folio
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Número de renovaciones
                if (window.event.keyCode==32)window.event.keyCode=8;
            
        
    
     
    
        
                        
                Tipo de negocio            
            
            
                 
            
            
                
                    DirectoCoaseguro aceptadoReaseguro aceptado
                
            
            
                 
            
                        
                Tipo de póliza            
            
            
                 
              
            
                
                    IndividualColectivoMultilocalidad
                
            
            
                 
            
            
                
                    
                                    
                            Relaciones            
                        
                        
                            
                          
                        
                            Contabilización
                            
                        
                    
                
            
            
                                                
                                    
                        
                                        
                                Modificación            
                            
                            
                                        
                              
                            
                                Tipo de endoso
                                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valType_amend.CanShowValues=true

var Parameters_valType_amend= new Object;
 var valType_amend1= new Object;
valType_amend1.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valType_amend1.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valType_amend1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valType_amend1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valType_amend1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valType_amend1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param1=valType_amend1;
 var valType_amend2= new Object;
valType_amend2.sName=&quot; , &quot;'&quot; , &quot;nProduct&quot; , &quot;'&quot; , &quot;;
valType_amend2.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valType_amend2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valType_amend2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valType_amend2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valType_amend2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param2=valType_amend2;
 var valType_amend3= new Object;
valType_amend3.sName=&quot; , &quot;'&quot; , &quot;dEffecdate&quot; , &quot;'&quot; , &quot;;
valType_amend3.sValue=&quot; , &quot;'&quot; , &quot;23/02/2023&quot; , &quot;'&quot; , &quot;;
valType_amend3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend3.sParType=&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;;
valType_amend3.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param3=valType_amend3;
var RParameters_valType_amend= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valType_amend.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valType_amend;
Parameters_valType_amend.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valType_amend;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;Tabtype_amend&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;]);


                                
                            
                            
                                F.E.R.
                                
                            
                        
                    
                
            
        
    


    
        
            Afectar certificados
        
    


    


    
        
            
                            
                    Fecha de vencimiento            
                
                
                            
                                        
                
                    Fecha
                    
                
            
        
    

insInitialFields(true,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
    


/html[1]/body[1]&quot;) or . = concat(&quot;Navegación Menú principal Transacción anterior Ir Salir del sistema Procesos batchAcciones Entrar Aceptar Cancelar Finalizar RefrescarAyuda Ayuda Ultimos valores Acerca de...

document.images[&quot; , &quot;'&quot; , &quot;A208&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A304&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A390&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A391&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A392&quot; , &quot;'&quot; , &quot;].belongtoolbar=truedocument.images[&quot; , &quot;'&quot; , &quot;A393&quot; , &quot;'&quot; , &quot;].belongtoolbar=true

if (top.frames[&quot;fraSequence&quot;].plngMainAction==0){
insHandImage(&quot;A208&quot;, true);
insHandImage(&quot;A301&quot;, true);
insHandImage(&quot;A302&quot;, true);
insHandImage(&quot;A303&quot;, true);
insHandImage(&quot;A304&quot;, true);
insHandImage(&quot;A305&quot;, true);
insHandImage(&quot;A306&quot;, true);
insHandImage(&quot;A310&quot;, true);
insHandImage(&quot;A401&quot;, true);
insHandImage(&quot;A402&quot;, true);
insHandImage(&quot;A390&quot;, false);
insHandImage(&quot;A391&quot;, false);
insHandImage(&quot;A392&quot;, false);
insHandImage(&quot;A393&quot;, false);
insHandImage(&quot;A394&quot;, true);
}
ClientRequest(304);
setPointer(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
top.document.title=&quot; , &quot;'&quot; , &quot;Tratamiento de pólizas (Company ABC)&quot; , &quot;'&quot; , &quot;;
var nMainAction=top.frames[&quot; , &quot;'&quot; , &quot;fraSequence&quot; , &quot;'&quot; , &quot;].plngMainAction




    
    
    
    

//- Variable para indicar si se limpian campos
    var mblnCleanField

//- Variable qie contiene el tipo enumerado para identificar el tipo de compañía
    var eCompanyType = new eCompanyType()
    
//var eTypeAction  = new TypeAction()

//- Variable que almacena el tipo de la compañía usuaria
    var mstrCompanyType = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la oficina del usuario de tipo intermediario que maneja la página
    var mintUserOffice = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la agencia del usuario de tipo intermediario que maneja la página
    var mintUserOfficeAgen = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
//- Variable que almacena la sucursal del usuario de tipo intermediario que maneja la página
    var mintUserAgency = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    
// - Variable para definir el tipo de documento (póliza, Solicitud, Cotización)
    var mstrCertype

//-Indicador de carga de menu
  var mintMenu = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  

//-Descripciones del campo póliza según transaccion
    var mstrPolicyDescript1 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Póliza&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript4 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Cotización&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript6 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Propuesta&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;
    var mstrPolicyDescript43 = &quot; , &quot;'&quot; , &quot;&lt;LABEL>Propuesta de Rehabilitación&lt;/LABEL>&quot; , &quot;'&quot; , &quot;;

//-Correspondencia Javascript de variables VbScript
    var mdtmEffecdate   = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintBranch      = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintProduct     = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintPolicy      = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintProponum    = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    var mintCertificat  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;





    
        
                        
             Clave    
            
            
                  
            
                Transacción
                
                    Aprobar solicitud de endosoConsulta de CertificadosConsulta de CotizaciónConsulta de PólizaConsulta de PropuestaConsulta Prop RehabilitaciónConv.Cotiz.Modi. a Prop. Modi.Conv.Cotiz.Modif. a Modif Nor.Convertir Cotiz. Renov. a Pól.Convertir Cotización a PólizaConvertir Cotización a Prop.Convertir Prop.  Renov. a Pól.Convertir Propuesta a PólizaCotización de CertificadosCotización de endoso(Certif)Cotización de endoso(Póliza)Cotización de Renov. Certif.Cotización de Renov. de PólizaCotización PólizaDuplicar PólizaEmisión CertificadoEmisión de PólizaEndoso a la PólizaEndoso al CertificadoModificación Temporal de Cert.Modificación Temporal de Pól.Propuesta de Renov. Certif.RecuperaciónRe-Emision de CertificadoRe-Emision de PólizaRe-ImpresiónSolicitud de endoso (Certif.)Solicitud de endoso (Póliza)Solicitudes de Emis - CertifSolicitudes de Emis - PólSolictud de Renov. de PólizaTraspaso de asegurado
            
            
                Fecha de vigencia
                var mstrDoSubmit = &quot;1&quot;;
            
            
                Sucursal
                
                Casa Central
                
            
            
                Oficina
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeOfficeAgen.CanShowValues=true

var Parameters_cbeOfficeAgen= new Object;
 var cbeOfficeAgen1= new Object;
cbeOfficeAgen1.sName=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeOfficeAgen.Param1=cbeOfficeAgen1;
 var cbeOfficeAgen2= new Object;
cbeOfficeAgen2.sName=&quot; , &quot;'&quot; , &quot;nAgency&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeOfficeAgen2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeOfficeAgen.Param2=cbeOfficeAgen2;
var RParameters_cbeOfficeAgen= new Object;
 var RcbeOfficeAgen1= new Object;
RcbeOfficeAgen1.Name=&quot; , &quot;'&quot; , &quot;nBran_off&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeOfficeAgen1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeOfficeAgen.Param1=RcbeOfficeAgen1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_cbeOfficeAgen.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeOfficeAgen;
Parameters_cbeOfficeAgen.nCount=2;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeOfficeAgen;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabAgencies_T5556&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeOfficeAgen&quot; , &quot;'&quot; , &quot;]);


                
            
            
                Agencia
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 
Casa Central


document.forms[0].cbeAgency.CanShowValues=true

var Parameters_cbeAgency= new Object;
 var cbeAgency1= new Object;
cbeAgency1.sName=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeAgency1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param1=cbeAgency1;
 var cbeAgency2= new Object;
cbeAgency2.sName=&quot; , &quot;'&quot; , &quot;nAgency&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
cbeAgency2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param2=cbeAgency2;
 var cbeAgency3= new Object;
cbeAgency3.sName=&quot; , &quot;'&quot; , &quot;nUsercode&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sValue=&quot; , &quot;'&quot; , &quot;9794&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sPrecision=&quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;;
cbeAgency3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_cbeAgency.Param3=cbeAgency3;
var RParameters_cbeAgency= new Object;
 var RcbeAgency1= new Object;
RcbeAgency1.Name=&quot; , &quot;'&quot; , &quot;nBran_off&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param1=RcbeAgency1;
 var RcbeAgency2= new Object;
RcbeAgency2.Name=&quot; , &quot;'&quot; , &quot;nOfficeAgen&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency2.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param2=RcbeAgency2;
 var RcbeAgency3= new Object;
RcbeAgency3.Name=&quot; , &quot;'&quot; , &quot;sDesAgen&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Visible=&quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Title=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
RcbeAgency3.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.Param3=RcbeAgency3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_cbeAgency.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_cbeAgency;
Parameters_cbeAgency.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_cbeAgency;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabAgencies_T5555A&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;cbeAgency&quot; , &quot;'&quot; , &quot;]);


                
                
            
            
                Canal de venta
                    
                     IntermediarioTELEMARKETINGDirectoWEBLICITACIÓNSucursales                    
                                                                        
            
                Ramo
                
function InsChangecbeBranch(nBranch){
    with(self.document.forms[0]){
        if (typeof(valProduct) != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){
            valProduct.disabled=(nBranch==&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;||nBranch==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;?true:false);
            btnvalProduct.disabled=valProduct.disabled;
            valProduct.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            UpdateDiv(&quot; , &quot;'&quot; , &quot;valProductDesc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            valProduct.Parameters.Param1.sValue = nBranch;
        }
    }
}

Comb. Familiar e IntegralesRobo y riesgos similaresVida colectivoRiesgos variosAccidentes PersonalesVida ObligatorioVida IndividualSaludSepelioGanado 
             
            
                Producto
                


if (window.event.keyCode==32)window.event.keyCode=8; document.btnvalProduct.disabled=true
Proteccion Integral


document.forms[0].valProduct.CanShowValues=true

var Parameters_valProduct= new Object;
 var valProduct1= new Object;
valProduct1.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valProduct1.sValue=&quot; , &quot;'&quot; , &quot;-32768&quot; , &quot;'&quot; , &quot;;
valProduct1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valProduct1.sParType=&quot; , &quot;'&quot; , &quot;8&quot; , &quot;'&quot; , &quot;;
valProduct1.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valProduct1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valProduct1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valProduct1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valProduct.Param1=valProduct1;
var RParameters_valProduct= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valProduct.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valProduct;
Parameters_valProduct.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valProduct;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;tabProdMaster1&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valProduct&quot; , &quot;'&quot; , &quot;]);


            
            
                
                Póliza 

                  
                if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;
document.btntcnPolicy.disabled=true

self.document.forms[0].elements[&quot; , &quot;'&quot; , &quot;tcnPolicy&quot; , &quot;'&quot; , &quot;].CertypeQuery=2;
function insShowPolicyQuerytcnPolicy(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp(&quot; , &quot;'&quot; , &quot;/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=&quot; , &quot;'&quot; , &quot; + FieldName + &quot; , &quot;'&quot; , &quot;&amp;FieldBranch=&quot; , &quot;'&quot; , &quot; + FieldBranch + &quot; , &quot;'&quot; , &quot;&amp;FieldProduct=&quot; , &quot;'&quot; , &quot; + FieldProduct + &quot; , &quot;'&quot; , &quot;&amp;FieldCertif=&quot; , &quot;'&quot; , &quot; + FieldCertif + &quot; , &quot;'&quot; , &quot;&amp;nBranch=&quot; , &quot;'&quot; , &quot; + BranchValue + &quot; , &quot;'&quot; , &quot;&amp;nProduct=&quot; , &quot;'&quot; , &quot; + ProductValue + &quot; , &quot;'&quot; , &quot;&amp;TypeList=&quot; , &quot;'&quot; , &quot; + TypeList + &quot; , &quot;'&quot; , &quot;&amp;List=&quot; , &quot;'&quot; , &quot; + List + &quot; , &quot;'&quot; , &quot;&amp;sCertypeQuery=&quot; , &quot;'&quot; , &quot; + self.document.forms[0].tcnPolicy.CertypeQuery, &quot; , &quot;'&quot; , &quot;ControldePoliza&quot; , &quot;'&quot; , &quot;, 750 , 480, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, 20, 20)
}

				
            
            
				Póliza destino
			    
			    if (window.event.keyCode==32)window.event.keyCode=8;-if (window.event.keyCode==32)window.event.keyCode=8;


self.document.forms[0].elements[&quot; , &quot;'&quot; , &quot;tcnPolicyDest&quot; , &quot;'&quot; , &quot;].CertypeQuery=2;
function insShowPolicyQuerytcnPolicyDest(FieldName, FieldBranch, FieldProduct, FieldCertif, BranchValue, ProductValue, TypeList, List){
ShowPopUp(&quot; , &quot;'&quot; , &quot;/VTimeNet/Common/PopUp.aspx?Type=PopUp&amp;sPageName=PolicyQuery&amp;FieldPolicy=&quot; , &quot;'&quot; , &quot; + FieldName + &quot; , &quot;'&quot; , &quot;&amp;FieldBranch=&quot; , &quot;'&quot; , &quot; + FieldBranch + &quot; , &quot;'&quot; , &quot;&amp;FieldProduct=&quot; , &quot;'&quot; , &quot; + FieldProduct + &quot; , &quot;'&quot; , &quot;&amp;FieldCertif=&quot; , &quot;'&quot; , &quot; + FieldCertif + &quot; , &quot;'&quot; , &quot;&amp;nBranch=&quot; , &quot;'&quot; , &quot; + BranchValue + &quot; , &quot;'&quot; , &quot;&amp;nProduct=&quot; , &quot;'&quot; , &quot; + ProductValue + &quot; , &quot;'&quot; , &quot;&amp;TypeList=&quot; , &quot;'&quot; , &quot; + TypeList + &quot; , &quot;'&quot; , &quot;&amp;List=&quot; , &quot;'&quot; , &quot; + List + &quot; , &quot;'&quot; , &quot;&amp;sCertypeQuery=&quot; , &quot;'&quot; , &quot; + self.document.forms[0].tcnPolicyDest.CertypeQuery, &quot; , &quot;'&quot; , &quot;ControldePoliza&quot; , &quot;'&quot; , &quot;, 750 , 480, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;, 20, 20)
}

			
			
                Asegurado
                
                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valCertif.CanShowValues=true

var Parameters_valCertif= new Object;
 var valCertif1= new Object;
valCertif1.sName=&quot; , &quot;'&quot; , &quot;sCertype&quot; , &quot;'&quot; , &quot;;
valCertif1.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif1.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif1.sSize=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif1.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param1=valCertif1;
 var valCertif2= new Object;
valCertif2.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valCertif2.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param2=valCertif2;
 var valCertif3= new Object;
valCertif3.sName=&quot; , &quot;'&quot; , &quot;nProduct&quot; , &quot;'&quot; , &quot;;
valCertif3.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif3.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif3.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif3.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param3=valCertif3;
 var valCertif4= new Object;
valCertif4.sName=&quot; , &quot;'&quot; , &quot;nPolicy&quot; , &quot;'&quot; , &quot;;
valCertif4.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif4.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif4.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valCertif4.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif4.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif4.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valCertif4.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param4=valCertif4;
 var valCertif5= new Object;
valCertif5.sName=&quot; , &quot;'&quot; , &quot;sClient&quot; , &quot;'&quot; , &quot;;
valCertif5.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valCertif5.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif5.sParType=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valCertif5.sSize=&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;;
valCertif5.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif5.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif5.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param5=valCertif5;
 var valCertif6= new Object;
valCertif6.sName=&quot; , &quot;'&quot; , &quot;dEffecdate&quot; , &quot;'&quot; , &quot;;
valCertif6.sValue=&quot; , &quot;'&quot; , &quot;23/02/2023&quot; , &quot;'&quot; , &quot;;
valCertif6.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valCertif6.sParType=&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;;
valCertif6.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valCertif6.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valCertif.Param6=valCertif6;
var RParameters_valCertif= new Object;
 var RvalCertif1= new Object;
RvalCertif1.Name=&quot; , &quot;'&quot; , &quot;nCertif&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Visible=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Title=&quot; , &quot;'&quot; , &quot;Certificado&quot; , &quot;'&quot; , &quot;;
RvalCertif1.Create=&quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;;
RParameters_valCertif.Param1=RvalCertif1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valCertif.nCount=1;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valCertif;
Parameters_valCertif.nCount=6;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valCertif;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;TabCertif&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valCertif&quot; , &quot;'&quot; , &quot;]);


                
			
						    
                Certificado
                
                if (window.event.keyCode==32)window.event.keyCode=8;document.btnPolicyValues.disabled=true
                
            
            
                
                
                Cotización / Propuesta
                
                
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Propuesta regularizada
                if (window.event.keyCode==32)window.event.keyCode=8;
            
                
                Número de inspección
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Folio
                if (window.event.keyCode==32)window.event.keyCode=8;
            
            
                Número de renovaciones
                if (window.event.keyCode==32)window.event.keyCode=8;
            
        
    
     
    
        
                        
                Tipo de negocio            
            
            
                 
            
            
                
                    DirectoCoaseguro aceptadoReaseguro aceptado
                
            
            
                 
            
                        
                Tipo de póliza            
            
            
                 
              
            
                
                    IndividualColectivoMultilocalidad
                
            
            
                 
            
            
                
                    
                                    
                            Relaciones            
                        
                        
                            
                          
                        
                            Contabilización
                            
                        
                    
                
            
            
                                                
                                    
                        
                                        
                                Modificación            
                            
                            
                                        
                              
                            
                                Tipo de endoso
                                


if (window.event.keyCode==32)window.event.keyCode=8; 



document.forms[0].valType_amend.CanShowValues=true

var Parameters_valType_amend= new Object;
 var valType_amend1= new Object;
valType_amend1.sName=&quot; , &quot;'&quot; , &quot;nBranch&quot; , &quot;'&quot; , &quot;;
valType_amend1.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valType_amend1.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend1.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valType_amend1.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valType_amend1.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend1.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valType_amend1.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param1=valType_amend1;
 var valType_amend2= new Object;
valType_amend2.sName=&quot; , &quot;'&quot; , &quot;nProduct&quot; , &quot;'&quot; , &quot;;
valType_amend2.sValue=&quot; , &quot;'&quot; , &quot;VT_EMPTY&quot; , &quot;'&quot; , &quot;;
valType_amend2.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend2.sParType=&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;;
valType_amend2.sSize=&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;;
valType_amend2.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend2.sPrecision=&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;;
valType_amend2.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param2=valType_amend2;
 var valType_amend3= new Object;
valType_amend3.sName=&quot; , &quot;'&quot; , &quot;dEffecdate&quot; , &quot;'&quot; , &quot;;
valType_amend3.sValue=&quot; , &quot;'&quot; , &quot;23/02/2023&quot; , &quot;'&quot; , &quot;;
valType_amend3.sDirection=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
valType_amend3.sParType=&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;;
valType_amend3.sSize=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sNumericScale=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sPrecision=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
valType_amend3.sAttributes=&quot; , &quot;'&quot; , &quot;64&quot; , &quot;'&quot; , &quot;;
Parameters_valType_amend.Param3=valType_amend3;
var RParameters_valType_amend= new Object;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].TypeList=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].List=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].TypeOrder=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
RParameters_valType_amend.nCount=0;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].RParameters =RParameters_valType_amend;
Parameters_valType_amend.nCount=3;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].Parameters =Parameters_valType_amend;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].sTabName=&quot; , &quot;'&quot; , &quot;Tabtype_amend&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].LookupAssembly=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;].LookupClass=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
SetParameters(document.forms[0].elements[&quot; , &quot;'&quot; , &quot;valType_amend&quot; , &quot;'&quot; , &quot;]);


                                
                            
                            
                                F.E.R.
                                
                            
                        
                    
                
            
        
    


    
        
            Afectar certificados
        
    


    


    
        
            
                            
                    Fecha de vencimiento            
                
                
                            
                                        
                
                    Fecha
                    
                
            
        
    

insInitialFields(true,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
    


/html[1]/body[1]&quot;))]</value>
      <webElementGuid>038435ba-2f16-4648-8c14-45a88995ed06</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
