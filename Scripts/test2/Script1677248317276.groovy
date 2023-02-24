import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

WebUI.callTestCase(findTestCase('Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('BuscarTransaccion'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC)')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/select_CasaCentral'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/input_Oficina_cbeOfficeAgen'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/input_Agencia_cbeAgency'), '1')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/select_ComboRamo'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/input_Producto_valProduct'), '9100')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/body_MenuPrincipal'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/label_Individual'))

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/btnGuardarCheck'))

WebUI.waitForPageLoad(10)

WebUI.navigateToUrl('http://ori-dtp-vtimeos.retiro.ar:8083/VTimeNet/Common/secWHeader.aspx?sCodispl=CA001&sProject=PolicySeq&sModule=Policy&sConfig=InSequence&bMenu=1')

WebUI.delay(2)

WebUI.refresh()

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a_Contratante'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/Page_/input_Cdigo del cliente_tctCode (1)'), '42933160')

WebUI.click(findTestObject('Page_/divPopUp'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Page_/img_Razn Social a imprimir_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Asegurables')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a_Ejecutivocomercial'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/Page_/input_Cdigo del cliente_tctCode (1)'), '21132149')

WebUI.click(findTestObject('Page_/divPopUp'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Page_/img_Razn Social a imprimir_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Asegurables')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a'))

WebUI.waitForPageLoad(10)

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/img_Titular_btntctClient'))

WebUI.click(findTestObject('Object Repository/Page_/a_00000042933160'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Titular_tctClient'), 
    '')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/td_var mintTypeForm2Allo , Camdela'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Titular_tctClient'), 
    '42933160')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/td_var mintTypeForm2'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/select_Facturacin vencidaNo AplicaFacturaci_fc9188'), 
    '2', true)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Convenio_valAgreement'), 
    '25')

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Nmero de comercio_valCommernum'), 
    '2829030')

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Convenio colectivo_valgroup_Agree'), 
    '25')

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Datos')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/a'))

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a'))

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/input_Nmero_valcredi_card'), 
    '1234123412341234')

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / V')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/a'))

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/input_Fecha de vencimiento_tcdDateExpir'), 
    '')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/a_22'))

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/input_Fecha de vencimiento_tcdDateExpir'), 
    '22/02/2024')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a_1'))

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/a'))

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/textarea_Calle_txtAddress'), 
    'Test')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/input_Nmero_tctBuild'), 
    '123')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/input_Cdigo postal_valZipCode'), 
    '1089')

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Dirección de la póliza')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/select_NoquiereinformarNotiene'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/a'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/select_ComisinfijaEstructuradegestinNoPoseeComisionSegntabla'), 
    '4', true)

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Intermediarios')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/img_Plan de comisin_btnvalCommPlan'))

WebUI.click(findTestObject('Object Repository/Page_Valores posibles/a_SUCREDITO86'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Intermediarios')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/a'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Intermediarios')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/img_Convenio de cobranza_cmdAdd'))

