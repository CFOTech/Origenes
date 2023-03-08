import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
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

WebUI.closeWindowTitle('VisualTIME 7.0')

WebUI.delay(2)

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC)')

WebUI.selectOptionByLabel(findTestObject('Page_Tratamiento de plizas (Company ABC)/selectTipoTransaccion'), 'Consulta de Póliza', 
    false)

WebUI.setText(findTestObject('Page_Tratamiento de plizas (Company ABC)/fechaPoliza'), '08/03/2023')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/select_ComboRamo'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/input_Producto_valProduct'), '9100')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/body_MenuPrincipal'))

WebUI.setText(findTestObject('Page_Tratamiento de plizas (Company ABC)/inputPoliza'), GlobalVariable.nroPoliza)

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/body_MenuPrincipal'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)/btnGuardarCheck'))

WebUI.waitForPageLoad(10)

WebUI.takeScreenshot()

