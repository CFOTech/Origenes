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

//DIR

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/textarea_Calle_txtAddress'),
	'Test')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/input_Nmero_tctBuild'),
	'123')

WebUI.setText(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/input_Cdigo postal_valZipCode'),
	'1089')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  V/body_Vadecobro    mstrSrvDecSep  , mstrUsrD_42bf5a'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  D_07a0ec/select_NoquiereinformarNotiene'),
	'1', true)

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/a'))