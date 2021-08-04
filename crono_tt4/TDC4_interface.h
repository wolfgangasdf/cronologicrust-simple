//
// Header file containing structs and #defines commonly used by header files of derived TDC4 device classes
// The current driver version for xTDC4/TimeTagger4 devices is 1.2.0.0
//

/*! \file
*	\brief The functions provided by the API are declared in xTDC4_interface.h/TimeTagger4_interface.h.
*
*	The API is a DLL with C linkage. There exists also .Net wrapper
*/ 
#ifndef TDC4_INTERFACE_H
#define TDC4_INTERFACE_H

#include "crono_interface.h"

/*! \defgroup constants Constants
*/
/*!	\defgroup initialization Initialization 
*@{
*/
/*! \defgroup initfuncts Functions for Initialization
* @{
*/
/*! \defgroup defclose #defines for xtdc4_close()
*
*	xtdc4_close() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*! \defgroup definit #defines for xtdc4_init()
*
*	xtdc4_init() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*/
/*! \defgroup defdefinpar #defines for xtdc4_get_default_init_parameters()
*
*	xtdc4_get_default_init_parameters() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*/
/*!@}*/
/*! \defgroup device Structure xtdc4_device
*/
/*!	\defgroup initparamsstruct Structure xtdc4_init_parameters
*	\brief struct for the initialization of the xTDC4
*
*	this structure MUST be completely INITIALIZED
*	@{
*/
/*!	\defgroup apiversion #define for version
*/
/*! \defgroup buffertype #defines for buffer_type
*  \brief type of buffer
*/
/*! \defgroup devicetype #define for device_type
*/
/*!@}*/
/*!@}*/
/*!	\defgroup status Status Information
*	@{
*/
/*! \defgroup statfuncts Functions for Information Retrieval
*	\brief Functions for getting detailed information about the xtdc4 board.
*
*	The driver provides functions to retrieve detailed information on the type of board,
*	it's configuration, settings and state. The information is split according to its scope
*	and the computational requirements to query the information from the board.
*@{
*/
/*!	\defgroup defparaminfo #defines for xtdc4_get_param_info()
*
*	xtdc4_get_param_info() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup defstatinfo #defines for xtdc4_get_static_info()
*
*	xtdc4_get_static_info() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup deffastinfo #defines for xtdc4_get_fast_info()
*
*	xtdc4_get_fast_info() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup defslowinfo #defines for xtdc4_get_slow_info()
*
*	xtdc4_get_slow_info() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!@}*/
/*!	\defgroup staticinfo Structure xtdc4_static_info
*	\brief Structure contains static information
*
*	This structure contains information about the board that does not change during run time.
*	It is provided by the function @link statfuncts xtdc4_get_static_info() @endlink.
*/
/*! \defgroup paraminfo Structure xtdc4_param_info
*	\brief contains configuration changes
*
*	Structure filled by @link statfuncts xtdc4_get_param_info() @endlink. This structure
*	contains information that change indirectly due to configuration changes.
*/
/*!	\defgroup fastinfo Structure xtdc4_fast_info
*	\brief contains fast dynamic information
*
*	This call returns a structure that contains dynamic information that can be obtained
*	within a few microseconds. 
*/
/*!@}*/
/*! \defgroup configuration Configuration
*	\brief Configuration of XTDC4
*	@{
*/
/*!	\defgroup conffuncts Configuration Functions
*	\brief Functions for the configuration of XTDC4
*
*	The device is configured with a configuration structure. The user should first obtain a structure	
*	that contains the default settings of the device read from an on board ROM, then modify the
*	structure as needed for the user application and use the result to configure the device.
* @{
*/
/*! \defgroup defconf #defines for xtdc4_configure()
*	
*	xtdc4_configure() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*! \defgroup defdefconf #defines for xtdc4_get_default_configuration()
*	
*	xtdc4_get_default_configuration() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*! \defgroup defcurconf #defines for xtdc4_get_current_configuration()
*	
*	xtdc4_get_current_configuration() returns one of the listed values, else
*	- @link funcerrors XTDC4_OK @endlink
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!@}*/
/*! \defgroup confstruct Structure xtdc4_configuration
*	\brief	contains configuration information
*
*	This structure contains the configuration information. It is used in conjunction with
*	@link conffuncts xtdc4_get_default_configuration(), xtdc4_get_current_configuration()
*	and xtdc4_configure() @endlink
*
*	Internally it uses the structures @link trigger xtdc4_trigger @endlink and @link tiger
*	xtdc4_tiger_block @endlink
* @{
*/
/*!	\defgroup tdcmode #defines for tdc_mode
*	\brief tdc_mode can be either grouped or continuous
*/
/*!	\defgroup defdcoffset #defines for dc_offset
*	\brief dc_offset values for various signal standards
*
*	used by @link xtdc4_configuration xtdc4_configuration @endlink.
*/
/*!@}*/
/*! \defgroup trigger Structure xtdc4_trigger
*	\brief contains trigger settings
*/
/*! \defgroup tiger Structure xtdc4_tiger_block
*	\brief contains settings of timing generator
* @{
*/
/*!	\defgroup  deftriggersource #defines for sources
*	\brief	mask for choosing the trigger source
*/
/*!@}*/
/*! \defgroup channel Structure xtdc4_channel
*	\brief contains TDC channel settings
*/
/*!@}*/
/*! \defgroup runtime Run Time Control
*	\brief control functions during run time
*	@{
*/
/*!	\defgroup defstartcap #defines for xtdc4_start_capture()
*	
*	xtdc4_start_capture() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup defstopcap #defines for xtdc4_stop_capture()
*	
*	xtdc4_stop_capture() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup defpausecap #defines for xtdc4_pause_capture()
*	
*	xtdc4_pause_capture() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!	\defgroup defcontcap #defines for xtdc4_continue_capture()
*	
*	xtdc4_continue_capture() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!@}*/
/*!	\defgroup mainread Readout
*	@{
*/
/*! \defgroup readout Functions for Readout
*	\brief functions for data readout
*	@{
*/
/*! \defgroup defack #defines for xtdc4_acknowledge()
*	
*	xtdc4_acknowledge() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*! \defgroup defread #defines for xtdc4_read()
*	
*	xtdc4_read() returns one of the listed values, else
*	- @link funcerrors XTDC4_INVALID_DEVICE @endlink
*/
/*!@}*/
/*!	\defgroup readin Structure xtdc4_read_in
*	\brief The parameters of the read commands
*/
/*! \defgroup strucreadout Structure xtdc4_read_out
* @{
*/
/*! \defgroup deferror #defines for error_code
*	\brief error code
*/
/*!@}*/
/*!@}*/
/*!	\defgroup funcerrors Function Errors
*
*	All ERRORS must be positive, because the upper byte is used by crono_tools
*/
/*! \defgroup packflags	Packet Flags
*	\brief flags of the packet reporting error conditions
*/

/*! \ingroup apiversion	*/
#define TDC4_API_VERSION 0
/*! \brief the number of analog input channels
*
* used by
* - @link confstruct xtdc4_configuration::dc_offset @endlink
* - @link confstruct xtdc4_configuration::channel @endlink
*/
#define TDC4_TDC_CHANNEL_COUNT 4
/*! \brief the number of digital input channels
*
* used by
* - @link confstruct xtdc4_configuration::lowres_channel @endlink
*/
#define TDC4_LOWRES_CHANNEL_COUNT 3
/*! \brief the number of timing generators
*
* used by
* - @link confstruct xtdc4_configuration::tiger_block[] @endlink
*/
#define TDC4_TIGER_COUNT 5
/*! \brief the number of triggers.
*
*	Two per analog input, one per digital input plus some specials.
*
*	used by
*	- @link confstruct xtdc4_configuration::trigger[] @endlink
*/
#define TDC4_TRIGGER_COUNT 16		// including "config" for AUTO and ONE
/*!@}*/

/*! \brief Valid values for tdc4_init_parameters.rclk_sel.
*/
#define TDC4_RCLK_37M5 0
#define TDC4_RCLK_75M 1
#define TDC4_RCLK_150M 2

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32) || defined(_WIN64)
#ifdef XTDC4_DRIVER_EXPORTS
#define XTDC4_API __declspec(dllexport)
#else
#define XTDC4_API __declspec(dllimport)
#endif
#else
#define __int64 long long
#define XTDC4_API 
#endif

#define crono_bool_t unsigned char

	/*!	\ingroup initparamsstruct 
	*	\brief struct for the initialization of the xTDC4
	*
	*	this structure MUST be completely INITIALIZED
	*/
	typedef struct {
		/* that is increased when the definition of the structure is changed. The increment can be larger than one
		*	to match driver version numbers or similar. Set to 0 for all versions up to first release.
		*/
		/*! \brief the version number
		*
		*	must be set to @link apiversion XTDC4_API_VERSION @endlink
		*/
		int version;

		/*! \brief The index in the list of XTDC4 boards that should be initialized.
		* 
		*	There might be multiple boards in the system that are handled by this driver as reported by @link
		*	initialization ndigo_count_devices() @endlink. This index selects one of them. Boards are enumerated
		*	depending on the PCIe slot. The lower the bus number and the lower the slot number the lower the card
		*	index.
		*/
		int card_index;

		/*! \brief the global index in all cronologic devices
		*
		*	This 8 bit number is filled into each packet created by the board and is useful if data streams of
		*	multiple boards will be merged. If only XTDC4 cards are used this number can be set to the
		*	@link card_index card_index @endlink. If boards of different types that use a compatible data format
		*	are used in a system each board should get a unique id.
		*/
		int board_id;

		/*!	\brief The minimum size of the DMA buffer.
		*
		*	If set to 0 the default size of 16MB is used.
		*	For the xtdc4 only the first entry is used.
		*/
		__int64 buffer_size[8];

		/*!	\brief The type of buffer
		*
		*	Can be either allocated (only option currently) or physical.
		*	Please refer to group @link buffertype #defines of buffer_type @endlink.
		*/
		int buffer_type;

		/*!	\brief The start address of the reserved memory.
		*
		*	The buffers will be allocated with the sizes given above. Make sure that the memory
		*	is large enough.
		*/
		__int64 buffer_address;

		/*! \brief A variant, for reconfiguring the chip for future extension 
		*
		*/
		int variant;

		/*! \brief A constant for the different devices of cronologic CRONO_DEVICE_*.
		*
		*	Initialized by @link initialization xtdc4_get_default_init_parameters() @endlink.
		*	Must be left unchanged. Please refer to @link devicetype #define for device_type @endlink.
		*/
		int device_type;

		/*! \brief The update delay of the writing pointer after a packet has been send over PCIe.
		*
		*	The base unit is 16 to 32 ns.
		*/
		int dma_read_delay;

		/*! \brief Select external 10 MHz reference.
		*
		*  If set to 1 use external 10 MHz reference.
		*  If set to 0 use internal reference.
		*/
		int use_ext_clock;

		/*! \brief Set THS788 RClk frequency, default is 150 MHz.
		* 
		* 0: 37.5 MHz
		* 1: 75 MHz
		* 2: 150 MHz
		*/
		int rclk_sel;
	} tdc4_init_parameters;


	/*! \ingroup staticinfo
	*	\brief Structure contains static information
	*
	*	This structure contains information about the board that does not change during run time.
	*	It is provided by the function @link statfuncts xtdc4_get_static_info() @endlink.
	*/
	typedef struct {

		int size; //!< The number of bytes occupied by the structure
		/*	A version number that is increased when the definition of the structure is changed. 
		*	The increment can be larger than one to match driver version numbers or similar. 
		*	Set to 0 for all versions up to first release.
		*/
		int version; //!< The version number
		/*!	\brief ID of the board.
		*
		*	This value is passed to the constructor. It is reflected in the	output data.
		*/
		int board_id;
		/*!	\brief encoded version number
		*
		*	The lower three bytes contain a triple level hierarchy of version numbers. 
		*	E.g. 0x010103 codes version 1.1.3.
		*	
		*	A change in the first digit generally requires
		*	a recompilation of user applications. Change in the second digit denote significant 
		*	improvements or changes that don't break compatibility and the third digit changes
		*	with minor bugfixes and the like.
		*/
		int driver_revision;
		/*	This can be read from a register.
		*/
		int firmware_revision; //!< Revision number of the FPGA configuration.
		/*!	\brief board revision number
		*
		*	The board revision number can be read from a register. It is a four bit number
		*	that changes when the schematic of the board is changed.
		*	- 0: Experimental first board Version. Labeled "Rev. 1"
		*	- 2: First commercial Version. Labeled "Rev. 2"
		*/
		int board_revision;
		/*!	\brief Describes the schematic configuration of the board.
		*
		*	The same board schematic can be populated in multiple variants. This is a eight
		*	bit code that can be read from a register.
		*/
		int board_configuration;
		/*	A number to track builds of the firmware in more detail than the firmware revision. It changes
		*	with every change in the firmware, even if there is no visible effect for the user. The subversion
		*	revision number can be read from a register. 
		*/
		int subversion_revision; //!< Subversion revision id of the FPGA configuration.
		/*  This is the chipID as read from the 16 bit ths788 chip id register at SPI address 0x83.
		*	This value should be cached.
		*/
		int chip_id; //!< 16bit factory ID of the TDC chip.
		/*!	\brief Serial number
		*	
		*	with year and running number in 8.24 format. The number is identical to the one
		*	printed on the silvery sticker on the board.
		*/
		int board_serial;
		unsigned int flash_serial_low;	//!< low 32 bits of 64 bit manufacturer serial number of the flash chip
		unsigned int flash_serial_high;	//!< high 32 bits of 64 bit manufacturer serial number of the flash chip
		/*!	\brief Flash data is valid and in use
		*	
		*	If not 0 the driver found valid calibration data in the flash on the board and is using it.
		*/
		int flash_valid;

		/*! \brief calibration date
		*
		* DIN EN ISO 8601 string YYYY-MM-DD HH:DD describing the time when the card was calibrated.
		*/
		char calibration_date[20];
	} tdc4_static_info;

	/*! \ingroup fastinfo
	*	\brief contains fast dynamic information
	*
	*	This call returns a structure that contains dynamic information that can be obtained
	*	within a few microseconds. 
	*/
	typedef struct {
		int size; //!< The number of bytes occupied by the structure
		/*	A version number that is increased when the definition of the structure is changed. 
		The increment can be larger than one to match driver version numbers or similar. Set
		to 0 for all versions up to first release.
		*/
		int version; //!< The version number
		/*! \brief Speed of the TDC fan.
		*
		*	Reports 0, if no fan is present
		*/
		int tdc_rpm;
		/*! \brief Speed of the FPGA fan.
		*
		* Reports 0, if no fan is present.
		*/
		int fpga_rpm;
		/*!	\brief Alert bits from temperature sensor and the system monitor
		*
		*	- bit 0:	THS788 over temperature alarm
		*/
		int alerts;
		/*! \brief organizes power supply of PCIe lanes.
		*/
		int pcie_pwr_mgmt;
		int pcie_link_width; //!< Number of PCIe lanes the card uses.
		/*!	\brief Maximum size for a single PCIe transaction
		*  in bytes. Depends on system configuration.
		*/
		int pcie_max_payload;
	} tdc4_fast_info;


	/*! \ingroup paraminfo
	*	\brief contains configuration changes
	*
	*	Structure filled by @link statfuncts xtdc4_get_param_info() @endlink. This structure
	*	contains information that change indirectly due to configuration changes.
	*/
	typedef struct {
		int size; //!< The number of bytes occupied by the structure
		/* is increased when the definition of the structure is changed. 
		The increment can be larger than one to match driver version numbers or similar. Set
		to 0 for all versions up to first release.
		*/
		int version; //!< The version number
		/*!	\brief Binsize (in ps) of the measured TDC data. 
		*
		*	The TDC main clk is running at a frequency of 76.8 GHz 
		*  resulting in a binsize of ~13.0208 ps.
		*/
		double binsize;
		/*!	\brief Board ID
		*
		*	The board uses this ID to identify itself in the output data stream. The ID takes
		*	values between 0 and 255.
		*/
		int board_id;
		/*!	\brief Number of channels in the current TDC mode
		*
		* Currently fixed at 4.
		*/
		int channels;
		/*! \brief bit assignment of each enabled input channel.
		*	
		*	Mask assigns a certain bit to each enabled input channel.
		*/
		int channel_mask;
		__int64 total_buffer; //!< The total amount of DMA buffer in bytes.
	} tdc4_param_info;

	/*!	\ingroup channel
	*	\brief Contains TDC channel settings
	*/
	typedef struct {
		crono_bool_t enabled;			//!< Enable TDC channel. 
		crono_bool_t rising;			//!< Set whether to record rising or falling edges.
		crono_bool_t cc_enable;			//!< Enable carry chain TDC as backup
		crono_bool_t cc_same_edge;		//!< Set whether the carry chain TDC records the same edge as THS788 (as backup) or opposite edge
		crono_bool_t ths788_disable;	//!< Disable THS788 timestamps

		/*!	\brief Veto function 
		*
		*	only timestamps >= start are recorded.
		*/
		int start;
		/*!	\brief Veto function
		*
		*	only timestamps <= stop are recorded. 
		*/
		int stop;

	} tdc4_channel;

	/*!	\ingroup lowres_channel
	*	\brief Contains digital channel settings
	*/
	typedef struct {
		crono_bool_t enabled; //!< Enable TDC channel.
		/*!	\brief Veto function 
		*
		*	only timestamps >= start are recorded.
		*/
		int start;
		/*!	\brief Veto function
		*
		*	only timestamps <= stop are recorded. 
		*/
		int stop;
	} tdc4_lowres_channel;

	/*!	\ingroup tiger
	*	\brief contains settings of timing generator
	*/
	typedef struct {
		crono_bool_t enable; //!< activates timing generator
		/*! \brief inverts output polarity
		*
		* default is set to false.
		*/
		crono_bool_t negate;
		/*!	\brief enables/disables retrigger setting
		*
		*	Default is set to false. If retriggering is enabled the timer is reset to the value of the start
		*	parameter, whenever the input signal is set while waiting to reach the stop time.
		*/
		crono_bool_t retrigger;
		// default is true
		crono_bool_t extend;				//!< not implemented
		crono_bool_t enable_lemo_output;	//!< enables the LEMO output
		/*! \brief Precursor
		*  
		*  Number of 6.6ns clock cycles before the tiger output goes active
		*	relative to the trigger signal.
		*/
		int start;
		/*! \brief postcursor
		*  
		*  Number of 6.6ns clock cycles before the tiger output goes inactive
		*	relative to the trigger signal.
		*/
		int stop;
		/*!	\brief mask for choosing the trigger source
		*
		*	A bit mask with a bit set for all trigger sources that can trigger this channel. Default is
		*	XTDC4_TRIGGER_SOURCE_S. One can choose a from a source @link deftriggersource here @endlink.
		*/
		int sources;
	} tdc4_tiger_block;

	/* low res trigger config */
	/*!\ingroup trigger
	*	\brief contains trigger settings
	*/
	typedef struct {
		crono_bool_t falling;	//!< triggers on falling edges
		crono_bool_t rising;	//!< triggers on rising edges
	} tdc4_trigger;


	/*! \ingroup confstruct Structure xtdc4_configuration
	*	\brief	contains configuration information
	*
	*	This structure contains the configuration information. It is used in conjunction with
	*	@link conffuncts xtdc4_get_default_configuration(), xtdc4_get_current_configuration()
	*	and xtdc4_configure() @endlink
	*
	*	Internally it uses the structures @link trigger xtdc4_trigger @endlink and @link tiger
	*	xtdc4_tiger_block @endlink
	*/
	typedef struct {
		/*! \brief The number of bytes occupied by the structure
		*/
		int size; //!< The number of bytes occupied by the structure
		/*! \brief A version number
		*
		* that is increased when the definition of the structure is changed. The increment
		* can be larger than one to match driver version numbers or similar. Set to 0 for
		* all versions up to first release.
		*/
		int version;
		/*! \brief TDC mode
		*
		*	can be @link tdcmode grouped or continous @endlink. Currently supported: grouped.
		*/
		int tdc_mode;
		/*! \brief rising or falling edge trigger
		*
		*	whether to sync the TDC on the rising or falling edge
		*/
		crono_bool_t start_rising;
		/*! \brief Set DAC channels for T, A - D.
		*	
		*	dc_offset[0]     : Start
		*
		*	dc_offset[1 - 4] : A - D
		*	
		*	Set to a value between -1.65V and +0.85V. This should be close to 50% of the height of your pulses on
		*	the inputs. Examples for various signaling standards are defined in @link defdcoffset #defines for
		*	dc_offset @endlink. The inputs are AC coupled. This means that for pulse inputs the absolute voltage is
		*	not important. it is rather the relative pulse amplitude that causes the input circuits to switch.
		*	dc_offset for an input must be set to the relative switching voltage for the input standard in use. If
		*	the pulses are negative, a negative switching threshold must be set and vice versa.
		*/
		double dc_offset[TDC4_TDC_CHANNEL_COUNT + 1];
		tdc4_trigger trigger[TDC4_TRIGGER_COUNT]; //!< Configuration of external trigger sources
		tdc4_tiger_block tiger_block[TDC4_TIGER_COUNT]; //!< configuration of the timing generator
		tdc4_channel channel[TDC4_TDC_CHANNEL_COUNT]; //!< configure polaritiy, type and threshold for the TDC channels
		tdc4_lowres_channel lowres_channel[TDC4_LOWRES_CHANNEL_COUNT]; //!< configure polaritiy, type and threshold for the digital channels
		/** \brief component to create a trigger either periodically or randomly.
		*
		*  To be exact, there are two parameters M = @link auto_trigger_period auto_trigger_period @endlink
		*	and N = @link auto_trigger_random_exponent auto_trigger_random_exponent @endlink
		*	that result in a distance between triggers of
		*
		*		T = 1 + M + [1...2^N]
		*
		*	clock cycles.
		*
		*		0 <= M < 2^32
		*
		*		0 <= N < 32
		*
		*  there is no enable or reset as the usage of this trigger can be configured in the channels.
		*/
		///@{
		int auto_trigger_period;
		/** \brief component to create a trigger either periodically or randomly.
		*
		*  To be exact, there are two parameters M = @link auto_trigger_period auto_trigger_period @endlink
		*	and N = @link auto_trigger_random_exponent auto_trigger_random_exponent @endlink
		*	that result in a distance between triggers of
		*
		*		T = 1 + M + [1...2^N]
		*
		*	clock cycles.
		*
		*		0 <= M < 2^32
		*
		*		0 <= N < 32
		*
		*  there is no enable or reset as the usage of this trigger can be configured in the channels.
		*/
		int auto_trigger_random_exponent;
		///@}

	} tdc4_configuration;


	/*!	\ingroup readin
	*	\brief The parameters of the read commands
	*/
	typedef struct {
		crono_bool_t acknowledge_last_read; //!< xtdc4_read automatically acknowledges packets from the last read
	} tdc4_read_in ;

	/*! \ingroup strucreadout
	*	\brief struct for the read out of the Ndigo packets
	*/
	typedef struct {
		/*!	\brief pointer to the first packet
		*
		*	that was captured by the call of @link readout xtdc4_read @endlink
		*/
		crono_packet *first_packet;
		/*!	\brief The packet after the last one
		*
		*	this is not a valid packet
		*/
		crono_packet *last_packet;
		/*! \brief error code
		*
		*	The assignments of the error codes can be found @link deferror here @endlink.
		*/
		int error_code;
		/*!	\brief error message
		*/
		const char *error_message;
	} tdc4_read_out;
#ifdef __cplusplus
}
#endif

#endif
