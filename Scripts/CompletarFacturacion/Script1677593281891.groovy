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

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Titular_tctClient'), 
    '42933160')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/td_var mintTypeForm2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/select_Facturacin vencidaNo AplicaFacturaci_fc9188'), 
    '2', true)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Convenio_valAgreement'), 
    '25')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a'))

WebUI.delay(1)

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Nmero de comercio_valCommernum'), 
    '2829030')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a'))

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/input_Convenio colectivo_valgroup_Agree'), 
    '25')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a'))

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  Datos/a'))

