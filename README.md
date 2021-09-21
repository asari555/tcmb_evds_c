# tcmb_evds_c

A C library for reaching the database of The Central Bank of The Republic of Turkey (CBRT).

The crate was purely written in Rust language and provides C FFI functions to make the EVDS web services operations in C language. The output of the built format of this crate is the C library. In addition, prebuilt C library is served to users in the release section.

### Table of Contents

+ **[General Information](#general-information)**
+ **[Prebuilt Library](#prebuilt-library)**
+ **[Pre-requests](#pre-requests)**
+ **[Installation](#installation)**
+ **[Building C Library](#building-c-library)**
+ **[Documentation](#documentation)**
+ **[Enums and Structures](#enums-and-structures)**
+ **[Operational Functions](#operational-functions)**
+ **[Parameters](#parameters)**
+ **[Example](#example)**
+ **[Attention](#attention)**
+ **[References](#references)**

## General Information

tcmb_evds_c is a Rust crate, namely a library for C users. This crate includes a mechanism to make EVDS web services requests with a module and operational FFI functions. The module is called `evds_c` which contains enums and structures that the operational functions need.

The crate is designed to operate the FFI functions easily and safely with the help of safe Rust language. Because of that the functions are supplied with entities of `evds_c` module. This module provides enums are responsible for making error handling, specifying return format of EVDS response and frequency formulas for advanced currency operations. In addition to enums, the module contains structures that make input, and result handling easy. These enums and structures are used as an argument for FFI operational functions. 

There are sub-modules inside of the `evds_c` module. These are called `advanced_entities`, `common_entities` and `error_handling`. Related entities such as enums are located in the mentioned modules. However, these modules are not given in the header file. C users works with the header that contains all of the public enums and structures located in the mentioned modules. These entities should be use to supply appropriate arguments to the operational function parameters.

## Prebuilt Library

There is a ready built library served to users. Furthermore, the header called `tcmb_evds_c.h` and the dynamic library for Windows, MacOs and Unix extensions are given as zipped together in the release section. 

*If users want to build the library on their computers, the installation procedures below must be followed.*

## Pre-requests

### API Key

+ Each operational function requires a unique API key to reach EVDS web services. Therefore, users need to have an own API key. To get an API key please follow [`Getting an API Key`] instructions. 

### Rust and Cargo software

+ Please, install [`Rust and Cargo`] first if these are not installed before.

[`Getting an API Key`]: <https://evds2.tcmb.gov.tr/help/videos/User_Guide_to_Access_EVDS_Data_by_Using_Python.pdf>

## Installation

Please,

clone the repository,
```
git clone https://github.com/asari555/tcmb_evds_c
```

open the folder,
```
cd tcmb_evds_c
```

[`Rust and Cargo`]: <https://doc.rust-lang.org/cargo/getting-started/installation.html>

## Building C Library

The crate should be built to get the required header file and OS dependent library for C. The the header file and the library names are `tcmb_evds_c.h` and `libtcmb_evds_c.so` respectively. In addition the header file and the required library are located in `target` and `target/release` folders respectively.

Please, apply the below command into your terminal in the workspace `tcmb_evds_c`.
```
cargo build --release
```

After this command, users can copy the required files from the mentioned folders.

## Documentation

For users who are **curious** about the usage hierarchy, seeing obvious examples and details of the crate, please apply the below command in the workspace `tcmb_evds_c` to open the documentation in their browsers.
```
cargo doc --open
```

## Enums and Structures

### **Enums** 
	
Aim of using enum is to specify required variable with a name not a magical number.

+ **TcmbEvdsReturnFormat**

 	is used in operational functions as an argument and specifies return format of the EVDS response.

+ **TcmbEvdsReturnErrorC**

	gives opportunity to handle specified errors. It is used with result structure.

+ **TcmbEvdsAggregationType**

	is used in operational functions as an argument and specifies aggregation type `tcmb_evds_c_get_advanced_data` function.

+ **TcmbEvdsFormula**

	is used in operational functions as an argument and specifies formula for `tcmb_evds_c_get_advanced_data` function.

+ **TcmbEvdsDataFrequency**

	is used in operational functions as an argument and specifies data frequency for `tcmb_evds_c_get_advanced_data` function.

### **Structures**

+ **TcmbEvdsInput**

	includes a char pointer and the length of the C string to handle with Rust language.

+ **TcmbEvdsResult**

	includes a char pointer, the length of the Rust string and error type to handle an error in the case of a problem. The error returns `NoError` when the result returns response against request. Otherwise, it returns specific error type.

## Operational Functions

### *tcmb_evds_c_get_data*

This function corresponds `2.1. Level Values Requests` in the [`EVDS web services guide`]. In addition, most of the data series can be requested by this function with giving appropriate and valid argument to the `data_series` parameter.

### *tcmb_evds_c_get_advanced_data*

This function corresponds `2.2. The Most Commonly Used Series with Frequecy Formulas` in the [`EVDS web services guide`]. It is important to mention that this function only provides operation for currency data series with frequency formulas. The extension `Frequency Formulas` cause naming the function advanced comparing to `tcmb_evds_c_get_data`.

### *tcmb_evds_c_get_data_group*

This function corresponds `3. All Series Data By Given Data Group` in the [`EVDS web services guide`]. The appropriate and valid data group code should be supplied to the `data_group` parameter in order to get all series data.

### *tcmb_evds_c_get_categories*

This function corresponds `4.1. Category Service` in the [`EVDS web services guide`] and returns all main categories provided by EVDS.

### *tcmb_evds_c_get_advanced_data_group*

This function corresponds `4.2. Data Group Service ` in the [`EVDS web services guide`] and returns requested data groups in EVDS with given mode and code options. To learn what are mode and code, please follow [`EVDS web services guide`]. Also, the function is named 'advanced' due to additional options with respect to `tcmb_evds_c_get_data_group` function.

### *tcmb_evds_c_get_series_list*

This function corresponds `4.3. Series Service` in the [`EVDS web services guide`] and returns a series list composed of data group and data series code.

## Parameters

### **Date**

Date/s can be given as an argument with required format which is "13-12-2011" for single date or "13-12-2011, 13-12-2012", "13-12-2011,13-12-2012" for multiple dates. In contrast, the invalid date formats cause returning error. 

### **Rest of the Parameters**

It is expected that valid arguments are provided for the parameters. The valid and related parameter arguments are provided in [`EVDS web services guide`]. Please, read the guide before using the operational functions.

## Example 

The operational FFI functions have `ascii_mode` parameter that converts the EVDS response into ascii chars and English chars. It is not expected that the response includes non-utf8 chars. However, the ascii_mode converts non-utf8 characters into ' * ' in the case of occurring. Furthermore, the mode becomes active if it is true.

Users need to be sure that their API key is valid. If it is valid, "VALID_API_KEY" string slice should be exchanged with the valid API key in the examples.

*About Result:* 
*The result of a function can be requested data or an error message. It can be easily understood that the result includes error if it is checked via `tcmb_evds_c_is_error` function. Furthermore, it is possible that the error type can be checked without reading error message.*

Example usage of `tcmb_evds_c_get_data` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    TcmbEvdsInput data_series;
    TcmbEvdsInput date;
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
    bool ascii_mode;
     
    
    // value assignments.
    data_series.input_ptr = "TP.DK.USD.S";
    data_series.string_capacity = strlen(data_series.input_ptr);
     
    date.input_ptr = "13-12-2011";
    date.string_capacity = strlen(date.input_ptr);
     
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);
    return_format = Csv;
     
    ascii_mode = false;

     
    // requesting data.
    TcmbEvdsResult data_result = tcmb_evds_c_get_data(data_series, date, api_key, return_format, ascii_mode);
     
     
    // handling error and printing the result.
    if (!tcmb_evds_c_is_error(data_result)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(data_result) ? "true" : "false");

    char* data_result_message = calloc(data_result.string_capacity, sizeof(char));
    memmove(data_result_message, data_result.output_ptr, data_result.string_capacity * sizeof(char));

    printf("%s", data_result_message);
 
    free(data_result_message);
 
    return 0;
}

```

Example usage of `tcmb_evds_c_get_advanced_data` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    TcmbEvdsInput data_series;
    TcmbEvdsInput date;
    
    TcmbEvdsAggregationType aggregation_type;
    TcmbEvdsFormula formula;
    TcmbEvdsDataFrequency data_frequency;
    
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
    
    bool ascii_mode;
    
    
    // value assignments.
    data_series.input_ptr = "TP.DK.USD.A";
    data_series.string_capacity = strlen(data_series.input_ptr);
    
    date.input_ptr = "13-12-2011";
    date.string_capacity = strlen(date.input_ptr);
    
    aggregation_type = End;
    formula = Level;
    data_frequency = Monthly;
    
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);
    
    return_format = Json;
    
    ascii_mode = false;
    
     
    // requesting data.
    TcmbEvdsResult advanced_data_result = 
        tcmb_evds_c_get_advanced_data(
            data_series, 
            date, 
            aggregation_type, 
            formula, 
            data_frequency, 
            api_key, 
            return_format,
            ascii_mode
            );
    
    
    // handling error and printing the result.
    if (!tcmb_evds_c_is_error(advanced_data_result)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(advanced_data_result) ? "true" : "false");
     
    char* advanced_data_result_message = calloc(advanced_data_result.string_capacity, sizeof(char));
    memmove(
        advanced_data_result_message, 
        advanced_data_result.output_ptr, 
        advanced_data_result.string_capacity * sizeof(char)
        );
    
    printf("%s", advanced_data_result_message);

    free(advanced_data_result_message);

    return 0;
}

```

Example usage of `tcmb_evds_c_get_data_group` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    TcmbEvdsInput data_group;
    TcmbEvdsInput date;
     
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
 
    bool ascii_mode;
 
 
    // value assignments.
    data_group.input_ptr = "bie_yssk";
    data_group.string_capacity = strlen(data_group.input_ptr);
 
    date.input_ptr = "01-06-2017,07-09-2017";
    date.string_capacity = strlen(date.input_ptr);
 
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);
 
    return_format = Json;
 
    ascii_mode = false;
 
 
    // requesting data.
    TcmbEvdsResult data_group_result = 
        tcmb_evds_c_get_data_group(
            data_group, 
            date, 
            api_key, 
            return_format,
            ascii_mode
            );
 

    // handling error and printing the result.
    if (!tcmb_evds_c_is_error(data_group)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(data_group) ? "true" : "false");
   
    char* data_group_message = calloc(data_group.string_capacity, sizeof(char));
    memmove(data_group_message, data_group.output_ptr, data_group.string_capacity * sizeof(char));
    
    printf("%s", data_group_message);

    free(data_group_message);

    return 0;
}

```

Example usage of `tcmb_evds_c_get_categories` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
 
    bool ascii_mode;
 

    // value assignments.
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);
 
    return_format = Json;
 
    ascii_mode = false;
 
 
    // requesting data.
    TcmbEvdsResult categories = tcmb_evds_c_get_categories(api_key, return_format, ascii_mode);
 
 
    // handling error and printing the result.
    if (!tcmb_evds_c_is_error(categories)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(categories) ? "true" : "false");
    
    char* categories_message = calloc(categories.string_capacity, sizeof(char));
    memmove(categories_message, categories.output_ptr, categories.string_capacity * sizeof(char));
   
     printf("%s", categories_message);
 
    free(categories_message);

    return 0;
}

```

Example usage of `tcmb_evds_c_get_advanced_data_group` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    unsigned int mode;
    TcmbEvdsInput code;   
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
    bool ascii_mode;
    
    
    // value assignments.
    mode = 1;
    
    code.input_ptr = "bie_yssk";
    code.string_capacity = strlen(code.input_ptr);
     
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);
    
    return_format = Json;
    
    ascii_mode = false;
    
    
    // requesting data.
    TcmbEvdsResult advanced_data_group = tcmb_evds_c_get_advanced_data_group(mode, code, api_key, return_format, ascii_mode);
     
    
    // handling error and printing the result.
    if (!tcmb_evds_c_is_error(advanced_data_group)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(advanced_data_group) ? "true" : "false");
    
    char* advanced_data_group_message = calloc(advanced_data_group.string_capacity, sizeof(char));
    memmove(
        advanced_data_group_message, 
        advanced_data_group.output_ptr, 
        advanced_data_group.string_capacity * sizeof(char)
        );
   
    printf("%s", advanced_data_group_message);

    free(advanced_data_group_message);

    return 0;
}

```

Example usage of `tcmb_evds_c_get_series_list` operational function:

```C
#include "tcmb_evds_c.h"


int main() {

    // declaration of required arguments.
    TcmbEvdsInput code;   
    TcmbEvdsInput api_key;
    TcmbEvdsReturnFormat return_format;
    bool ascii_mode;
    
    
    // value assignments.
    code.input_ptr = "bie_yssk";
    code.string_capacity = strlen(code.input_ptr);;
    
    api_key.input_ptr = "VALID_API_KEY";
    api_key.string_capacity = strlen(api_key.input_ptr);;
    
    return_format = Csv;
    
    ascii_mode = false;
    
    
    // requesting data.
    if (!tcmb_evds_c_is_error(series_list)) { printf("\nNO ERROR!\n"); };
    printf("\nError: %s", tcmb_evds_c_is_error(series_list) ? "true" : "false");
   
    char* series_list_message = calloc(series_list.string_capacity, sizeof(char));
    memmove(series_list_message, series_list.output_ptr, series_list.string_capacity * sizeof(char));
    
    printf("%s", series_list_message);

    free(series_list_message);

    free(series_list.output_ptr);

    return 0;
}

```

[`EVDS web services guide`]: <https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf>

## Attention

Various errors are handled such as invalid input parameters, bad internet connection etc. given with `ReturnErrorC` enum elements. However, some unexpected invalid data series or data groups supplied by users may cause EVDS response error which could not be handled. Therefore, the result does not return an error, though the response includes an error message. 

In conclusion, users are responsible to provide valid arguments to operational functions. Especially, `data_series` and `data_group` parameters must be supplied appropriately and with valid arguments.

## References

+ **[EVDS user guide](https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf)**. CBRT EVDS. 

+ **[API key generation](https://evds2.tcmb.gov.tr/help/videos/User_Guide_to_Access_EVDS_Data_by_Using_Python.pdf)**. CBRT EVDS.
