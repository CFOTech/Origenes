import org.apache.poi.xssf.usermodel.XSSFWorkbook
import org.apache.poi.xssf.usermodel.XSSFSheet



def pathRelativo = 

FileInputStream fileInputPath = new FileInputStream(pathRelativo)
XSSFWorkbook wbook = new XSSFWorkbook(fileInputPath)