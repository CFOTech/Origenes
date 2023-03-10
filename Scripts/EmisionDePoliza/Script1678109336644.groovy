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

WebUI.callTestCase(findTestCase('CompletarMenuPrincipal'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarClientes'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarFacturacion'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarViaDeCobro'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarDireccion'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarProductores'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarModulo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarCoberturas'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarInfoRecibo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CompletarClausulas'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('GuardarIngreso'), [:], FailureHandling.STOP_ON_FAILURE)

