// Importing of the code
use super::starter;
use super::lexer;

// Lexer
use lexer::Lexer;
use lexer::LFuncs;
use lexer::Type;
use lexer::LError;
use lexer::LErrorFuncs;

// File Info
use starter::FileInfo;
use starter::FErrors;
use starter::ErrFuncs;
use starter::Funcs;

// Standard Imports
use std::path::PathBuf;
use std::env;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

impl From<io::Error> for FErrors
{
    fn from(err: io::Error) -> FErrors
    {
        FErrors::NoSuchDir(err)
    }
}

impl ErrFuncs for FErrors
{
    fn no_file(file: PathBuf) -> FErrors {
        FErrors::NoSuchFile(file)
    }
    fn file_creation_error(file: PathBuf) -> FErrors
    {
        FErrors::CreateFileErr(file)
    }
}

impl Funcs for FileInfo
{
    fn new() -> Result<FileInfo, io::Error>
    {
        let main_dir = env::current_dir()?;

        Ok(
            Self
            {
                dir: main_dir.to_path_buf(),
                is_valid: main_dir.exists(),
                content: String::new()
            }
        )
    }

    fn append(&mut self, filename: String) -> Result<FileInfo, FErrors>
    {
        self.dir = self.dir.join(filename);

        if self.dir.exists()
        {
            return Ok(self.clone());
        }

        Err(FErrors::NoSuchFile(self.dir.clone()))
    }

    fn read_file(&mut self) -> io::Result<FileInfo>
    {
        let content = BufReader::new(File::open(&self.dir)?);

        for line in content.lines()
        {
            let ln = line?;
            self.content.push_str(&ln);
        }
        Ok(self.clone())
    }
}

impl LFuncs for Lexer
{
    fn new_lexer(info: FileInfo) -> Self
    {
        Self {
            info: info,
            token: Type::Def,
            index: 0
        }
    }

    fn advance_with_token(&mut self, token: Type) -> Type
    {
        self.index += 1;
        return token;
    }

    fn skip_whitespace(&mut self)
    {
        self.index += 1;
    }

    fn lex(&mut self) -> Result<Type, LError>
    {
        let last_char = self.info.content.chars().last().unwrap();

        loop {
            match self.info.content.chars().nth(self.index) {
                Some(' ') => {
                    self.skip_whitespace();
                    continue;
                },
                Some('{') => {
                    return Ok(self.advance_with_token(Type::T_LB));
                }
                Some('}') => {
                    return Ok(self.advance_with_token(Type::T_RB));
                }
                Some(',') => {
                    return Ok(self.advance_with_token(Type::Comma));
                }
                Some('\t') => {
                    return Ok(self.advance_with_token(Type::Tab));
                }
                None => break,
                _ => {
                    match self.info.content.chars().nth(self.index).unwrap().is_digit(10) {
                        true => return Ok(self.advance_with_token(Type::NUM)),
                        false => {
                            break;
                        }
                    }
                }
            }
        }
        Ok(Type::EOF)
    }
}

impl LErrorFuncs for LError
{
    fn token_error(err: Type) -> LError
    {
        LError::TokenErr(err)
    }
}
